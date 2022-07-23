use eframe::egui;

#[derive(Default, Debug)]
struct App {}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Here goes image");
        });
    }
}

fn main() {
    let window_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Attractor",
        window_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
