use eframe::egui;

#[derive(Default, Debug)]
pub struct ImageWidget {}

impl ImageWidget {
  pub fn new() -> Self {
    Self::default()
  }
}

impl egui::Widget for ImageWidget {
  fn ui(self, ui: &mut egui::Ui) -> egui::Response {
    let response = ui.heading("Image goes here");
    response
  }
}
