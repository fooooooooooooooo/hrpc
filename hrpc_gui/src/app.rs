use std::time::{Duration, Instant};

use eframe::NativeOptions;
use egui::CentralPanel;
use hrpc::reading;

use crate::graph::Graph;

pub fn start() -> Result<(), eframe::Error> {
  let options = NativeOptions::default();

  eframe::run_native("hrpc", options, Box::new(|cc| Box::new(App::new(cc))))
}

pub struct App {
  graph: Graph,
  current_reading: u8,
  last_measurement: Instant,
}

impl App {
  pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
    Self {
      graph: Default::default(),
      current_reading: 0,
      last_measurement: Instant::now(),
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    CentralPanel::default().show(ctx, |ui| {
      if Instant::now() - self.last_measurement > Duration::from_millis(1000) {
        self.current_reading = reading::get();
        self.last_measurement = Instant::now();

        self.graph.new_point(self.current_reading);
      }

      ui.label(format!("reading: {}", self.current_reading));

      self.graph.show(ui, 200.0);
    });
  }
}
