use tokio::runtime::Runtime;
use tokio::time::interval;

use crate::config::Config;
use crate::template::Template;
use crate::{append, reading};

pub fn log_thread(config: Config) {
  tokio::task::block_in_place(|| {
    let rt = Runtime::new().unwrap();

    rt.block_on(async move {
      if let Err(e) = log_task(config).await {
        error!("log_task error: {}", e);
      }
    });
  })
}

async fn log_task(config: Config) -> anyhow::Result<()> {
  debug!("log_task start");
  if !config.log.enable {
    return Ok(());
  }

  let mut interval = interval(config.log.update_interval);

  loop {
    interval.tick().await;

    let mut template = Template::new(config.log.template.clone());
    template.add("reading", reading::get().to_string());
    template.add("timestamp", timestamp());

    append(&config.log.path, format!("{}\n", template.render())).await?;
  }
}

/// `1985-04-12T23:20:50`
fn timestamp() -> String { chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string() }
