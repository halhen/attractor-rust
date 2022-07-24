use std::collections::HashMap;
use super::raster::Raster;
use image::ImageBuffer;
use colorgrad::Gradient;

type ScalingFunction = dyn Fn(f64) -> f64;
pub enum Scaling {
  Binary,
  Linear,
  Sqrt,
  Log
}

fn scaler(scaling: Scaling) -> Box<ScalingFunction> {
  let function = match scaling {
    Scaling::Binary => |x| if x < 0.001 {0.} else {1.},
    Scaling::Linear => |x| x,
    Scaling::Sqrt => |x| f64::sqrt(x),
    Scaling::Log => |x| f64::log2(x + 1.),
  };

  Box::new(function)
}


pub fn render(raster: &Raster, scaling: Scaling, colors: Gradient) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
{
  let width = raster.width() as u32;
  let height = raster.height() as u32;

  let scale = scaler(scaling);
  let color = |intensity| {
    if intensity == 0. {
      image::Rgba([0, 0, 0, 255])
    } else {
      image::Rgba(colors.at(1.0 - intensity).to_rgba8())
    }
  };

  let image = ImageBuffer::from_fn(width, height, |x, y| {
    let intensity = scale(raster.intensity(x, y));
    color(intensity)
  });

  image
}
