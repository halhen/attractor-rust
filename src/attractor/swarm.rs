use super::point::Point;

#[derive(Debug)]
pub struct Swarm {
  points: Vec<Point>,
}

impl Swarm {
  pub fn new() -> Self {
    Self { points: vec![] }
  }

  pub fn add(&mut self, p: Point) {
    self.points.push(p);
  }

  pub fn points(&self) -> &Vec<Point> {
    &self.points
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn swarm_works() {
    let mut s = super::Swarm::new();
    s.add(super::Point { x: 1., y: 2. });
    assert_eq!(s.points().len(), 1);
    s.add(super::Point { x: 2., y: 1. });
    assert_eq!(s.points().len(), 2);
    s.add(super::Point { x: 0., y: 3. });
    assert_eq!(s.points().len(), 3);
  }
}