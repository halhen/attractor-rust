use std::collections::HashMap;
use super::raster::Raster;
use image::ImageBuffer;

type ScalingFunction = dyn Fn(f64) -> f64;
pub enum Scaling {
  BINARY,
  LINEAR,
  SQRT,
  LOG
}

fn scaler(scaling: Scaling) -> Box<ScalingFunction> {
  let function = match scaling {
    BINARY => |x| if x < 0.001 {0.} else {1.},
    LINEAR => |x| x,
    SQRT => |x| f64::sqrt(x),
    LOG => |x| f64::log2(x + 1.),
  };

  Box::new(function)
}


pub fn render(raster: &Raster, scaling: Scaling) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
{
  let width = raster.width() as u32;
  let height = raster.height() as u32;

  let scale = scaler(scaling);

  let image = ImageBuffer::from_fn(width, height, |x, y| {
    let intensity = scale(raster.intensity(x, y));
    let color = (255.0 * intensity) as u8;
    image::Rgba([color, color, color, 255])
  });

  image
}
