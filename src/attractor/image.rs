use super::raster::Raster;
use image::ImageBuffer;
use colorgrad::Gradient;

type ScalingFunction = dyn Fn(f64) -> f64;
#[derive(Debug, PartialEq, Copy, Clone, strum_macros::EnumIter)]
pub enum Scaling {
  Binary,
  Linear,
  Log,
  Sqrt,
  CubeRoot,
  FourthRoot,
  FifthRoot,
}

fn scaler(scaling: Scaling) -> Box<ScalingFunction> {
  let function = match scaling {
    Scaling::Binary => |x| if x < 0.001 {0.} else {1.},
    Scaling::Linear => |x| x,
    Scaling::Log => |x| f64::log2(x + 1.),
    Scaling::Sqrt => f64::sqrt,
    Scaling::CubeRoot  => |x| f64::powf(x, 1.0/3.0),
    Scaling::FourthRoot  => |x| f64::powf(x, 1.0/4.0),
    Scaling::FifthRoot  => |x| f64::powf(x, 1.0/5.0),
  };

  Box::new(function)
}


#[derive(Debug, PartialEq, Copy, Clone, strum_macros::EnumIter)]
pub enum Palette {
  Greys,
  Blues,
  Greens,
  Organges,
  Purples,
  Reds,
  
  BrBg,
  PrGn,
  PiYg,
  PuOr,
  RdBu,
  RdGy,
  RdYlBu,
  RdYlGn,
  BuGn,
  BuPu,
  GnBu,
  OrRd,
  PuBuGn,
  PuBu,
  PuRd,
  YlGnBu,
  YlGn,
  YlOrBr,
  YlOrRd,

  Turbo,
  Viridis,
  Inferno,
  Magma,
  Plasma,
  Cividis,
  Warm,
  Cool,

  CubehelixDefault,
  Spectral,
  Rainbow,
  Sinebow,
}

fn colorer(palette: Palette) -> Gradient {
  match palette {
    Palette::Greys => colorgrad::greys(),
    Palette::Blues => colorgrad::blues(),
    Palette::Greens => colorgrad::greens(),
    Palette::Organges => colorgrad::oranges(),
    Palette::Purples => colorgrad::purples(),
    Palette::Reds => colorgrad::reds(),
    
    Palette::BrBg => colorgrad::br_bg(),
    Palette::PrGn => colorgrad::pr_gn(),
    Palette::PiYg => colorgrad::pi_yg(),
    Palette::PuOr => colorgrad::pu_or(),
    Palette::RdBu => colorgrad::rd_bu(),
    Palette::RdGy => colorgrad::rd_gy(),
    Palette::RdYlBu => colorgrad::rd_yl_bu(),
    Palette::RdYlGn => colorgrad::rd_yl_gn(),
    Palette::BuGn => colorgrad::bu_gn(),
    Palette::BuPu => colorgrad::bu_pu(),
    Palette::GnBu => colorgrad::gn_bu(),
    Palette::OrRd => colorgrad::or_rd(),
    Palette::PuBuGn => colorgrad::pu_bu_gn(),
    Palette::PuBu => colorgrad::pu_bu(),
    Palette::PuRd => colorgrad::pu_rd(),
    Palette::YlGnBu => colorgrad::yl_gn_bu(),
    Palette::YlGn => colorgrad::yl_gn(),
    Palette::YlOrBr => colorgrad::yl_or_br(),
    Palette::YlOrRd => colorgrad::yl_or_rd(),

    Palette::Turbo => colorgrad::turbo(),
    Palette::Viridis => colorgrad::viridis(),
    Palette::Inferno => colorgrad::inferno(),
    Palette::Magma => colorgrad::magma(),
    Palette::Plasma => colorgrad::plasma(),
    Palette::Cividis => colorgrad::cividis(),
    Palette::Warm => colorgrad::warm(),
    Palette::Cool => colorgrad::cool(),

    Palette::CubehelixDefault => colorgrad::cubehelix_default(),
    Palette::Spectral => colorgrad::spectral(),
    Palette::Rainbow => colorgrad::rainbow(),
    Palette::Sinebow => colorgrad::sinebow(),
  }
}



pub fn render(raster: &Raster, scaling: Scaling, palette: Palette, palette_reverse: bool) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
{
  let width = raster.width() as u32;
  let height = raster.height() as u32;

  let scale = scaler(scaling);
  let colors = colorer(palette);

  let color = |intensity| {
    if intensity == 0. {
      image::Rgba([0, 0, 0, 255])
    } else if palette_reverse {
      image::Rgba(colors.at(1.0 - intensity).to_rgba8())
    } else {
      image::Rgba(colors.at(intensity).to_rgba8())
    }
  };

  ImageBuffer::from_fn(width, height, |x, y| {
    let intensity = scale(raster.intensity(x, y));
    color(intensity)
  })
}
