use eframe::egui;
use rand::Rng;
use crate::attractor::{self, quadratic2d};

#[derive(Debug)]
pub struct App {
    params: [f64; 12],

    // Cached results
    swarm: Option<attractor::swarm::Swarm>,
    raster: Option<attractor::raster::Raster>,
    image: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            params: [1., 0., -1.4, 0., 0.3, 0., 0., 1., 0., 0., 0., 0.],
            swarm: None,
            raster: None,
            image: None
        }
    }

    // 
    fn refresh_render(&mut self, width: usize, height: usize) {
         match self.swarm {
            Some(_) => {},
            None => self.swarm = Some(attractor::quadratic2d::generate(&self.params, 100_000))
        };

        match self.raster {
            Some(_) => {},
            None => self.raster = Some(attractor::raster::Raster::new(&self.swarm.as_ref().unwrap(), width, height))
        };

        match self.image {
            Some(_) => {},
            None => self.image = Some(attractor::image::render(
                &self.raster.as_ref().unwrap(),
                attractor::image::Scaling::LINEAR,
                colorgrad::greys()
            ))
        };
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            let button_response = ui.button("Randomize");
            if button_response.clicked() {
                println!("Randomize");
                self.swarm = None;
                self.raster = None;
                self.image = None;
                let mut rng = rand::thread_rng();

                for i in 0..self.params.len() {
                    self.params[i] = rng.gen_range(-1.5..=1.5);
                }
                println!("{:?}", self.params);
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
