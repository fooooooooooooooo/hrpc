use blehr::{Scanner, Sensor};
use futures_lite::StreamExt;
use tokio::runtime::Runtime;
use tokio::time::{sleep, timeout};

use crate::config::Config;
use crate::reading;

pub fn monitor_thread(config: Config) -> anyhow::Result<()> {
  tokio::task::block_in_place(|| {
    let rt = Runtime::new()?;

    rt.block_on(monitor_loop(&config));

    Ok(())
  })
}

async fn monitor_loop(config: &Config) {
  loop {
    if let Err(e) = monitor_task(config).await {
      error!("{:?}", e);

      sleep(config.restart_delay).await;
    }

    reading::set(reading::NONE);
  }
}

async fn monitor_task(config: &Config) -> anyhow::Result<()> {
  debug!("monitor_task start");

  let mut sensor = find_sensor(config).await?;

  let mut stream = sensor.hr_stream().await?;

  // todo: what if it is none forever
  while let Ok(Some(reading)) = timeout(config.read_timeout, stream.next()).await {
    debug!("reading: {:?}", reading);

    if let Some(reading) = reading {
      reading::set(reading);
    } else {
      reading::set(reading::NONE);
    }
  }

  Ok(())
}

async fn find_sensor(config: &Config) -> anyhow::Result<Sensor> {
  loop {
    debug!("scanning for sensors");

    let mut scanner = Scanner::new();
    scanner.start().await?;

    let sensor = timeout(config.scan_timeout, scanner.next_sensor()).await??;
    scanner.stop().await?;

    if let Some(sensor) = sensor {
      let name = sensor.name().await.unwrap_or_else(|| "unknown name".to_string());
      debug!("found sensor: {name}");
      return Ok(sensor);
    }

    debug!("no sensor found, retrying in {}ms", config.scan_retry_delay.as_millis());
    sleep(config.scan_retry_delay).await;
  }
}
