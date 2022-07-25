use eframe::egui;

pub struct ImageWidget<'a> {
  image: &'a Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>
}

impl<'a> ImageWidget<'a> {
  pub fn new(image: &'a Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>) -> Self {
    Self {
      image: image
    }
  }
}

impl<'a> Default for ImageWidget<'a> {
  fn default() -> Self {
    Self {
      image: &None
    }
  }
}

impl<'a> egui::Widget for ImageWidget<'a> {
  fn ui(self, ui: &mut egui::Ui) -> egui::Response {
    match self.image {
      None => {
        ui.label("No image yet")
      },
      Some(image) => {
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
        ui.image(texture, ui.available_size())
      }
    }
  }    
}
