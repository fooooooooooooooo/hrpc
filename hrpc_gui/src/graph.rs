use std::collections::VecDeque;

use egui::Vec2b;
use egui_plot::{Line, Plot, PlotBounds, PlotPoint, PlotPoints};

pub struct Graph {
  points: VecDeque<PlotPoint>,
  current_x: f64,
  scroll_locked: bool,
}

impl Graph {
  pub fn new() -> Self {
    Self {
      points: Default::default(),
      current_x: 0.0,
      scroll_locked: true,
    }
  }

  pub fn show(&mut self, ui: &mut egui::Ui, height: f32) {
    let points = PlotPoints::Owned(self.points.iter().cloned().collect::<Vec<_>>());

    let line = Line::new(points);

    ui.checkbox(&mut self.scroll_locked, "scroll locked");

    Plot::new("plot")
      .height(height)
      .allow_zoom(Vec2b::new(!self.scroll_locked, false))
      .allow_drag(Vec2b::new(!self.scroll_locked, false))
      .allow_scroll(false)
      .include_y(0.0)
      .include_y(255.0)
      .show_x(false)
      .show(ui, |ui| {
        ui.line(line);

        if self.scroll_locked {
          let min = (self.current_x - 100.0).max(-2.0);
          let max = self.current_x.max(100.0);

          ui.set_plot_bounds(PlotBounds::from_min_max([min, 0.0], [max, 255.0]));
        }
      });
  }

  pub fn new_point(&mut self, current_reading: u8) {
    self
      .points
      .push_back(PlotPoint::new(self.current_x, current_reading as f64));
    self.current_x += 1.0;

    if self.current_x > 10000.0 {
      self.points.pop_front();
    }
  }
}

impl Default for Graph {
  fn default() -> Self {
    Self::new()
  }
}
