#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x: x, y: y }
    }
}

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
}

#[derive(Debug)]
pub struct Raster {
    counts: Vec<f64>,
}

impl Raster {
    pub fn new(swarm: &Swarm, width: usize, height: usize) -> Self {
        let mut me = Self {
            counts: vec![0.0; width * height]
        };

        let xmin = swarm
            .points
            .iter()
            .fold(f64::INFINITY, |acc, p| acc.min(p.x));
        let xmax = swarm
            .points
            .iter()
            .fold(f64::NEG_INFINITY, |acc, p| acc.max(p.x));
        let xrange = xmax - xmin;
        let xmin = xmin - xrange * 1.05;
        let xmax = xmax + xrange * 1.05;
        let xstep = (xmax - xmin) / (width as f64);

        let ymin = swarm
            .points
            .iter()
            .fold(f64::INFINITY, |acc, p| acc.min(p.y));
        let ymax = swarm
            .points
            .iter()
            .fold(f64::NEG_INFINITY, |acc, p| acc.max(p.y));
        let yrange = ymax - ymin;
        let ymin = ymin - yrange * 1.05;
        let ymax = ymax + yrange * 1.05;
        let ystep = (ymax - ymin) / (height as f64);

        for point in &swarm.points {
            let x = ((point.x - xmin) / xstep).floor() as usize;
            let y = ((point.y - ymin) / ystep).floor() as usize;
            me.counts[x + y * width] += 1.0;
        }

        let max_count = me
            .counts
            .iter()
            .fold(0.0, |acc: f64, count| acc.max(*count));
        me.counts.iter_mut().for_each(|x| {
            *x /= max_count;
        });

        me
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn swarm_works() {
        let mut s = super::Swarm::new();
        s.add(super::Point { x: 1., y: 2. });
        assert_eq!(s.points.len(), 1);

        s.add(super::Point { x: 2., y: 1. });
        assert_eq!(s.points.len(), 2);

        s.add(super::Point { x: 0., y: 3. });
        assert_eq!(s.points.len(), 3);
    }

    #[test]
    fn raster_works() {
        let mut s = super::Swarm::new();
        s.add(super::Point { x: 1., y: 2. });
        s.add(super::Point { x: 2., y: 1. });
        s.add(super::Point { x: 0., y: 3. });
        s.add(super::Point { x: 0., y: 3. });

        let x = super::Raster::new(&s, 2, 2);
        assert_eq!(x.counts[0], 0.0);
        assert_eq!(x.counts[1], 0.5);
        assert_eq!(x.counts[2], 1.0); 
        assert_eq!(x.counts[3], 0.5);     }
}
