use rand::Rng;
use super::point::Point;
use super::raster::Raster;
use super::quadratic2d::{step, generate, Params};

pub fn exponent(params: &Params, iterations: usize) -> f64 {
  const EPSILON: f64 = 0.0000001;
  let component_epsilon = f64::sqrt(EPSILON * EPSILON / 2.);

  let mut point = Point::new(0.1, 0.1);
  let mut sum_log_distance_ratios = 0.0;

  for _ in 0..iterations {
    let mut point2 = Point::new(point.x + component_epsilon, point.y + component_epsilon);

    point = step(params, point);
    point2 = step(params, point2);

    let distance = (f64::powi(point.x - point2.x, 2) + f64::powi(point.y - point2.y, 2)).sqrt();
    if distance == 0.0 {
      return(f64::NEG_INFINITY);
    }

    if distance.abs() == f64::INFINITY {
      return(f64::INFINITY);
    }

    sum_log_distance_ratios += (distance / EPSILON).log2();
  }

  sum_log_distance_ratios / iterations as f64
}

pub fn random_chaotic_params() -> Params {
  let mut rng = rand::thread_rng();
  let mut candidate: Params = [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.];
  
  loop {
    for i in 0..candidate.len() {
      candidate[i] = rng.gen_range(-1.5..=1.5);
    }

    // Check that the lyapunov exponent indicates chaos
    let e = exponent(&candidate, 10_000);
    if e < 0.0 || e == f64::INFINITY {
      continue
    }

    // Generate test raster to make sure we get visually non-trivial results
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;
    // Minimum number of cells ever visited in a WIDTH * HEIGHT raster
    const LIMIT: usize = 200;

    let swarm = generate(&candidate, 10_000);
    let raster = Raster::new(&swarm, WIDTH, HEIGHT);

    let visited_cells: usize = raster.intensities().iter().fold(0, |acc, x| if *x > 0.0 {acc + 1} else {acc});
    if visited_cells < LIMIT {
      continue
    }

    println!("Suggesting {:?} with exponent {} and visited cells {}", candidate, e, visited_cells);
    return(candidate);
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn exponent_works() {

  }
}

