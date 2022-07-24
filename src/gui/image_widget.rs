use eframe::egui;
use crate::attractor::{quadratic2d::generate, raster::Raster, image::render, image::Scaling};
use colorgrad;

pub struct ImageWidget {
  
}

impl ImageWidget {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Default for ImageWidget {
  fn default() -> Self {
    Self {
    }
  }
}

impl egui::Widget for ImageWidget {
  fn ui(self, ui: &mut egui::Ui) -> egui::Response {
    let size = ui.available_size();
    let params = [-0.16, -0.37, -0.27, 0.16, -0.66, -0.74, -1.11, -0.51, 0.59, 0.81, -0.06, -0.44];
    let swarm = generate(&params, 1_000_000);
    let raster = Raster::new(&swarm, size.x as usize, size.y as usize);
    let image = render(
      &raster,
      Scaling::LINEAR,
      colorgrad::plasma()
    );

    let size = [image.width() as _, image.height() as _];
    let pixels: image::FlatSamples<&[u8]> = image.as_flat_samples();
    let gui_image = egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice()
    );
    let mut texture: Option<egui::TextureHandle> = None;
    let texture: &egui::TextureHandle = texture.get_or_insert_with(|| {
        ui.ctx().load_texture("attractor-img", gui_image)
    });
    let response = ui.image(texture, ui.available_size());
    response
  }
}
