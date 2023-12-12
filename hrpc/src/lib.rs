use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub mod config;
pub mod file;
pub mod logging;
pub mod monitor;
pub mod osc;
pub mod reading;
pub mod rpc;
pub mod template;

#[macro_use]
extern crate log;

pub async fn append(path: &str, data: String) -> anyhow::Result<()> {
  let mut file = OpenOptions::new().create(true).append(true).open(path).await?;

  file.write_all(data.as_bytes()).await?;

  futures_lite::future::block_on(file.flush())?;

  Ok(())
}

pub async fn overwrite(path: &str, data: String) -> anyhow::Result<()> {
  let mut file = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(path)
    .await?;

  file.write_all(data.as_bytes()).await?;

  futures_lite::future::block_on(file.flush())?;

  Ok(())
}
