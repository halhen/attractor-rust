mod gui;
mod attractor;

fn main() {
    //let a = attractor::quadratic2d::generate(&[0.1, 0.1, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0], 100_000);
    gui::run();
}
