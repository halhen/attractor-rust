use super::point::Point;
use super::swarm::Swarm;

pub type Params = [f64; 12];

pub fn step(params: &Params, from: Point) -> Point {
  Point::new(
    params[0] + params[1] * from.x + params[2] * from.x * from.x + params[3] * from.x * from.y + params[ 4] * from.y + params[ 5] * from.y * from.y,
    params[6] + params[7] * from.x + params[8] * from.x * from.x + params[9] * from.x * from.y + params[10] * from.y + params[11] * from.y * from.y
  )
}

pub fn generate(params: &Params, iterations: usize) -> Swarm {
  let mut point = Point::new(0.1, 0.1);
  let mut swarm = Swarm::new();
  for _ in 0..iterations {
    point = step(params, point);
    swarm.add(point);
  }
  swarm
}
