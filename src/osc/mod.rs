use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Context;
use rosc::encoder::encode;
use rosc::{OscMessage, OscPacket, OscType};
use tokio::net::UdpSocket;
use tokio::time::interval;

use crate::config::Config;
use crate::reading;

const INT_PATHS: &[&str] = &[
  "/avatar/parameters/HR",
  "/avatar/parameters/onesHR",
  "/avatar/parameters/tensHR",
  "/avatar/parameters/hundredsHR",
];
const FLOAT_PATH: &str = "/avatar/parameters/floatHR";
const PERCENT_PATH: &str = "/avatar/parameters/percentHR";
const ACTIVE_PATH: &str = "/avatar/parameters/isHRConnected";

pub fn osc_thread(config: Config) {
  tokio::task::block_in_place(|| {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
      if let Err(e) = osc_task(config).await {
        error!("osc_task error: {}", e);
      }
    });
  })
}

async fn osc_task(config: Config) -> anyhow::Result<()> {
  debug!("osc_task start");
  if !config.osc.enable {
    return Ok(());
  }

  let mut interval = interval(config.osc.update_interval);

  let host: IpAddr = config.osc.host.parse().context("failed to parse host")?;

  let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))
    .await
    .context("failed to bind to address")?;

  let addr = SocketAddr::new(host, config.osc.port);

  info!("osc ready");

  loop {
    interval.tick().await;

    let reading = reading::get();

    send_ints(&socket, addr, reading).await?;
    send_float(&socket, addr, reading).await?;
    send_percent(&socket, addr, reading, config.osc.percent_min, config.osc.percent_max).await?;
    send_active(&socket, addr, reading).await?;
  }
}

async fn send_ints(socket: &UdpSocket, addr: SocketAddr, reading: u8) -> anyhow::Result<()> {
  let values = [reading, reading % 10, reading / 10 % 10, reading / 100 % 10];

  for (path, value) in INT_PATHS.iter().zip(values.iter()) {
    let message = OscPacket::Message(OscMessage {
      addr: path.to_string(),
      args: vec![OscType::Int(*value as i32)],
    });

    let buf = encode(&message)?;
    socket.send_to(&buf, addr).await?;
  }

  Ok(())
}

async fn send_float(socket: &UdpSocket, addr: SocketAddr, reading: u8) -> anyhow::Result<()> {
  let float_hr = if reading == 0 {
    0.0
  } else {
    reading as f32 * 0.0078125 - 1.0
  };

  let message = OscPacket::Message(OscMessage {
    addr: FLOAT_PATH.to_string(),
    args: vec![OscType::Float(float_hr)],
  });

  let buf = encode(&message)?;
  socket.send_to(&buf, addr).await?;

  Ok(())
}

fn percent(reading: u8, min: u8, max: u8) -> f32 {
  if reading < min {
    return 0.0;
  }

  if reading > max {
    return 1.0;
  }

  (reading - min) as f32 / (max - min) as f32
}

async fn send_percent(socket: &UdpSocket, addr: SocketAddr, reading: u8, min: u8, max: u8) -> anyhow::Result<()> {
  let message = OscPacket::Message(OscMessage {
    addr: PERCENT_PATH.to_string(),
    args: vec![OscType::Float(percent(reading, min, max))],
  });

  let buf = encode(&message)?;
  socket.send_to(&buf, addr).await?;

  Ok(())
}

static ACTIVE: AtomicBool = AtomicBool::new(false);

async fn send_active(socket: &UdpSocket, addr: SocketAddr, reading: u8) -> anyhow::Result<()> {
  let active = reading != 0;

  if ACTIVE.load(Ordering::Relaxed) == active {
    return Ok(());
  }

  ACTIVE.store(active, Ordering::Relaxed);

  debug!("sending active: {}", active);

  let message = OscPacket::Message(OscMessage {
    addr: ACTIVE_PATH.to_string(),
    args: vec![OscType::Bool(active)],
  });

  let buf = encode(&message)?;
  socket.send_to(&buf, addr).await?;

  Ok(())
}
