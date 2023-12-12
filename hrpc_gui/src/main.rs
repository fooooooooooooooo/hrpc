// hide console window
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, thread};

use anyhow::{anyhow, Context};
use hrpc::config::load_config;
use hrpc::monitor::monitor_thread;
use hrpc_gui::app;

#[macro_use]
extern crate log;

fn main() -> anyhow::Result<()> {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info");
  }

  pretty_env_logger::init();

  info!("hello awa");

  let config = load_config().context("failed to load config from `config.toml`")?;

  // let osc_config = config.clone();
  // let osc = thread::spawn(move || osc_thread(osc_config));

  // let rpc_config = config.clone();
  // let rpc = thread::spawn(move || rpc_thread(rpc_config));

  // let file_config = config.clone();
  // let file = thread::spawn(move || file_thread(file_config));

  // let log_config = config.clone();
  // let log = thread::spawn(move || log_thread(log_config));

  let monitor = thread::spawn(move || monitor_thread(config));

  app::start().map_err(|err| anyhow!("{err}"))?;

  drop(monitor);

  Ok(())
}
