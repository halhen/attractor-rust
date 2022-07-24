mod gui;
mod attractor;

fn main() {
    //let a = attractor::quadratic2d::generate(, 100_000);
    //let e = attractor::lyapunov::exponent(&[1.03, -1.11, 0.25, -0.51, 0.6, 0.33, 1.2, 0.28, -0.71, -0.68, -0.66, -0.8], 100_000);
    //println!("{}", e);
    gui::run();
}
