use super::raster::Raster;
use image::ImageBuffer;

pub fn render<F>(raster: &Raster, scale: F) ->
    image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
    where
      F: Fn(f64) -> f64
{
  let width = raster.width() as u32;
  let height = raster.height() as u32;

  let image = ImageBuffer::from_fn(width, height, |x, y| {
    let intensity = scale(raster.intensity(x, y));
    let color = (255.0 * intensity) as u8;
    image::Rgba([color, color, color, 255])
  });

  image
}
