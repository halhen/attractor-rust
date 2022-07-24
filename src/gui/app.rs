use eframe::egui;
use crate::attractor::{self, quadratic2d};

#[derive(Debug)]
pub struct App {
    params: [f64; 12],
    // Average # iterations per pixel of render
    iterations_per_pixel: f64,

    // Cached results
    swarm: Option<attractor::swarm::Swarm>,
    raster: Option<attractor::raster::Raster>,
    image: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let style = egui::Style {
            visuals: egui::Visuals::dark(),
            ..egui::Style::default()
        };
        cc.egui_ctx.set_style(style);
        Self {
            params: [1., 0., -1.4, 0., 0.3, 0., 0., 1., 0., 0., 0., 0.],
            iterations_per_pixel: 1.0,
            swarm: None,
            raster: None,
            image: None
        }
    }

    // 
    fn refresh_render(&mut self, width: usize, height: usize) {
         match self.swarm {
            Some(_) => {},
            None => self.swarm = Some(attractor::quadratic2d::generate(&self.params, ((width * height) as f64 * self.iterations_per_pixel) as usize))
        };

        match self.raster {
            Some(_) => {},
            None => self.raster = Some(attractor::raster::Raster::new(&self.swarm.as_ref().unwrap(), width, height))
        };

        match self.image {
            Some(_) => {},
            None => self.image = Some(attractor::image::render(
                &self.raster.as_ref().unwrap(),
                attractor::image::Scaling::Sqrt,
                colorgrad::blues()
            ))
        };
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            if ui.button("Randomize").clicked() {
                println!("Randomize");
                self.swarm = None;
                self.raster = None;
                self.image = None;
                self.params = attractor::lyapunov::random_chaotic_params();
            }

            ui.add(egui::DragValue::new(&mut self.iterations_per_pixel).clamp_range(0.1..=100.0).speed(0.1));

            if ui.button("Redraw").clicked() {
                self.swarm = None;
                self.raster = None;
                self.image = None;
            }

            self.refresh_render(
                ui.available_width() as usize,
                ui.available_height() as usize
            );
            
            ui.add(super::image_widget::ImageWidget::new(&self.image));
        });
    }
}

pub fn run() {
  let window_options = eframe::NativeOptions::default();
  eframe::run_native(
      "Attractor",
      window_options,
      Box::new(|cc| Box::new(App::new(cc))),
  );
}
