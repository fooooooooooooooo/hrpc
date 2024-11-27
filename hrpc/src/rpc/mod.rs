use anyhow::anyhow;
use discord_rich_presence::activity::Activity;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use tokio::time::interval;

use crate::config::Config;
use crate::reading;
use crate::template::Template;

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

  let mut client = DiscordIpcClient::new(&config.rpc.id).map_err(ah)?;

  client.connect().map_err(ah)?;

  info!("rpc ready");

  let mut interval = interval(config.rpc.update_interval);

  loop {
    interval.tick().await;

    match reading::get() {
      reading::Reading::None => disconnected_activity(&mut client, &config)?,
      reading::Reading::Frozen(r) => frozen_activity(&mut client, &config, r)?,
      reading::Reading::Value(r) => activity(&mut client, &config, r)?,
    };
  }
}

fn activity(client: &mut DiscordIpcClient, config: &Config, reading: u8) -> anyhow::Result<()> {
  let mut state = Template::new(config.rpc.templates.state.clone());
  state.add("reading", reading.to_string());

  let details = config.rpc.templates.details.clone();
  let state = state.render();

  let activity = Activity::new().details(&details).state(&state);

  client.set_activity(activity).map_err(ah)
}

fn frozen_activity(client: &mut DiscordIpcClient, config: &Config, reading: u8) -> anyhow::Result<()> {
  let mut state = Template::new(config.rpc.templates.frozen_state.clone());
  state.add("reading", reading.to_string());

  let details = config.rpc.templates.frozen_details.clone();
  let state = state.render();

  client
    .set_activity(Activity::new().details(&details).state(&state))
    .map_err(ah)
}

fn disconnected_activity(client: &mut DiscordIpcClient, config: &Config) -> anyhow::Result<()> {
  let details = config.rpc.templates.disconnected_details.clone();
  let state = config.rpc.templates.disconnected_state.clone();

  client
    .set_activity(Activity::new().details(&details).state(&state))
    .map_err(ah)
}

fn ah(err: Box<dyn std::error::Error>) -> anyhow::Error {
  anyhow!("{:?}", err)
}
