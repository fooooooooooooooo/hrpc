use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::anyhow;
use discord_rpc_client::models::Activity;
use discord_rpc_client::Client;
use tokio::time::interval;

use crate::config::Config;
use crate::reading;
use crate::template::Template;

static READY: AtomicBool = AtomicBool::new(false);
static ERROR: AtomicBool = AtomicBool::new(false);

pub fn rpc_thread(config: Config) {
  tokio::task::block_in_place(|| {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
      if let Err(e) = rpc_task(config).await {
        error!("rpc_task error: {}", e);
      }
    });
  })
}

async fn rpc_task(config: Config) -> anyhow::Result<()> {
  debug!("rpc_task start");
  if !config.rpc.enable {
    return Ok(());
  }

  let mut client = Client::new(config.rpc.id);

  client.start();

  client.on_ready(|_| {
    READY.store(true, Ordering::Relaxed);
    info!("rpc ready")
  });

  client.on_error(|err| {
    error!("{:?}", err);
    ERROR.store(true, Ordering::Relaxed);
  });

  let mut interval = interval(config.rpc.update_interval);

  loop {
    interval.tick().await;

    if !READY.load(Ordering::Relaxed) {
      continue;
    }

    if ERROR.load(Ordering::Relaxed) {
      break Err(anyhow!("rpc client error"));
    }

    match reading::get() {
      0 => {
        client
          .set_activity(|a| na_activity(a, &config))
          .map_err(|err| anyhow!("{:?}", err))?;
      }
      reading => {
        client
          .set_activity(|a| activity(a, &config, reading))
          .map_err(|err| anyhow!("{:?}", err))?;
      }
    }
  }
}

fn na_activity(activity: Activity, config: &Config) -> Activity {
  Activity {
    details: Some(config.rpc.templates.na_details.clone()),
    state: Some(config.rpc.templates.na_state.clone()),
    ..activity
  }
}

fn activity(activity: Activity, config: &Config, reading: u8) -> Activity {
  let mut state = Template::new(config.rpc.templates.state.clone());
  state.add("reading", reading.to_string());

  Activity {
    details: Some(config.rpc.templates.details.clone()),
    state: Some(state.render()),
    ..activity
  }
}
