use std::time::Instant;

use anyhow::bail;
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

  info!(
    "connected to sensor: {}",
    sensor.name().await.unwrap_or_else(|| "unknown name".to_string())
  );

  let mut last_reading_time = Instant::now();

  loop {
    let reading = timeout(config.read_timeout, stream.next()).await?;

    let reading = match reading {
      Some(reading) => reading,
      None => {
        if last_reading_time.elapsed() > config.read_timeout {
          bail!("no reading received for {}ms", config.read_timeout.as_millis());
        }

        continue;
      }
    };

    debug!(
      "reading {}",
      reading.map(|x| x.to_string()).unwrap_or("None".to_owned())
    );

    if let Some(reading) = reading {
      reading::set(reading);
    } else {
      reading::set(reading::NONE);
    }

    last_reading_time = Instant::now();
  }
}

async fn find_sensor(config: &Config) -> anyhow::Result<Sensor> {
  debug!("scanning for sensors");
  let mut scanner = Scanner::new();
  scanner.start().await?;

  loop {
    let sensor = timeout(config.scan_timeout, scanner.next_sensor()).await??;
    // let sensor = scanner.next_sensor().await?;
    scanner.stop().await?;

    if let Some(sensor) = sensor {
      let name = sensor.name().await.unwrap_or_else(|| "unknown name".to_string());
      debug!("found sensor: {name}");
      return Ok(sensor);
    }

    // if let Some(sensor) = scanner.device_stream().next().await {
    //   let name = sensor.local_name().await.unwrap_or_else(|| "unknown
    // name".to_string());   debug!("found sensor: {name}");

    //   match Sensor::from_device(sensor).await {
    //     Ok(sensor) => return Ok(sensor),
    //     Err(e) => {
    //       warn!("sensor does not have compatible characteristic: {}", e);
    //       continue;
    //     }
    //   }
    // }

    warn!("no sensor found, retrying in {}ms", config.scan_retry_delay.as_millis());
    sleep(config.scan_retry_delay).await;
  }
}
