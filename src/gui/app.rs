use eframe::egui;
use strum::IntoEnumIterator;
use crate::attractor::{self};
use crate::attractor::image::{Scaling, Palette};

#[derive(Debug, Copy, Clone)]
pub struct AttractorSettings {
    params: [f64; 12],
    iterations_per_pixel: f64,
    width: usize,
    height: usize,

    scaling: Scaling,
    palette: Palette,
    palette_reverse: bool
}

impl AttractorSettings {
    pub fn new() -> Self {
        Self {
            params: [1., 0., -1.4, 0., 0.3, 0., 0., 1., 0., 0., 0., 0.],
            iterations_per_pixel: 1.0,
            width: 0,
            height: 0,
            scaling: Scaling::Linear,
            palette: Palette::Greys,
            palette_reverse: false
        }
    }
}

pub struct App {
    settings: AttractorSettings,

    // Cached results
    settings_rendered: Option<AttractorSettings>,
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
            settings: AttractorSettings::new(),
            
            settings_rendered: None,
            swarm: None,
            raster: None,
            image: None
        }
    }

    // 
    fn refresh_render(&mut self, width: usize, height: usize) {
        if let Some(settings_rendered) = self.settings_rendered {
            if settings_rendered.params != self.settings.params ||
               settings_rendered.iterations_per_pixel != self.settings.iterations_per_pixel ||
               settings_rendered.width != self.settings.width ||
               settings_rendered.height != self.settings.height {
                self.swarm = None;
                self.raster = None;
                self.image = None;
            }
        }

         match self.swarm {
            Some(_) => {},
            None => self.swarm = Some(attractor::quadratic2d::generate(&self.settings.params, ((width * height) as f64 * self.settings.iterations_per_pixel) as usize))
        };

        match self.raster {
            Some(_) => {},
            None => self.raster = Some(attractor::raster::Raster::new(&self.swarm.as_ref().unwrap(), width, height))
        };

        if let Some(settings_rendered) = self.settings_rendered {
            if settings_rendered.scaling != self.settings.scaling ||
               settings_rendered.palette != self.settings.palette ||
               settings_rendered.palette_reverse != self.settings.palette_reverse {
                self.image = None;
            }
        }

        match self.image {
            Some(_) => {},
            None => {
                self.image = Some(attractor::image::render(
                    &self.raster.as_ref().unwrap(),
                    self.settings.scaling,
                    self.settings.palette,
                    self.settings.palette_reverse
                ));
                
                self.settings_rendered = Some(self.settings);
            }
        };
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for i in 0..self.settings.params.len() {
                ui.add(egui::DragValue::new(&mut self.settings.params[i]).clamp_range(-1.5..=1.5).speed(0.001));
            }

            if ui.button("Randomize").clicked() {
                println!("Randomize");
                self.swarm = None;
                self.raster = None;
                self.image = None;
                self.settings.params = attractor::lyapunov::random_chaotic_params();
            }

            ui.add(egui::DragValue::new(&mut self.settings.iterations_per_pixel).clamp_range(0.1..=100.0).speed(0.1));
            
            egui::ComboBox::from_label("Scaling")
                .selected_text(format!("{:?}", self.settings.scaling))
                .show_ui(ui, |ui| {
                    for choice in Scaling::iter() {
                        ui.selectable_value(&mut self.settings.scaling, choice, format!("{:?}", choice));
                    }
                });

            egui::ComboBox::from_label("Palette")
                .selected_text(format!("{:?}", self.settings.palette))
                .show_ui(ui, |ui| {
                    for choice in Palette::iter() {
                        ui.selectable_value(&mut self.settings.palette, choice, format!("{:?}", choice));
                    }
                });

            ui.add(egui::Checkbox::new(&mut self.settings.palette_reverse, "Reverse palette"));
            
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
