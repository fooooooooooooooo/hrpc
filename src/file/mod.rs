use tokio::runtime::Runtime;
use tokio::time::interval;

use crate::config::Config;
use crate::template::Template;
use crate::{overwrite, reading};

pub fn file_thread(config: Config) {
  tokio::task::block_in_place(|| {
    let rt = Runtime::new().unwrap();

    rt.block_on(async move {
      if let Err(e) = file_task(config).await {
        error!("file_task error: {}", e);
      }
    });
  })
}

async fn file_task(config: Config) -> anyhow::Result<()> {
  debug!("file_task start");
  if !config.file.enable {
    return Ok(());
  }

  let mut interval = interval(config.file.update_interval);

  loop {
    interval.tick().await;

    let mut template = Template::new(config.file.template.clone());
    template.add("reading", reading::get().to_string());

    let rendered = template.render();

    debug!("file_task writing: {}", rendered);

    overwrite(&config.file.path, rendered).await?;
  }
}
