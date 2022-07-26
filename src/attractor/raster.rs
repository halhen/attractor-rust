use super::swarm::Swarm;

#[derive(Debug)]
pub struct Raster {
    density: Vec<f64>,

    width: usize,
    height: usize
}

impl Raster {
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn intensities(&self) -> &Vec<f64> {
        &self.density
    }

    pub fn intensity(&self, x: u32, y: u32) -> f64 {
        self.density[(x + y * self.width as u32) as usize]
    }

    pub fn new(swarm: &Swarm, width: usize, height: usize) -> Self {
        let mut me = Self {
            density: vec![0.0; width * height],
            width,
            height
        };

        let xmin = swarm
            .points()
            .iter()
            .fold(f64::INFINITY, |acc, p| acc.min(p.x));
        let xmax = swarm
            .points()
            .iter()
            .fold(f64::NEG_INFINITY, |acc, p| acc.max(p.x));
        let xrange = xmax - xmin;
        let xmin = xmin - xrange * 0.1;
        let xmax = xmax + xrange * 0.1;
        let xstep = (xmax - xmin) / (width as f64);

        let ymin = swarm
            .points()
            .iter()
            .fold(f64::INFINITY, |acc, p| acc.min(p.y));
        let ymax = swarm
            .points()
            .iter()
            .fold(f64::NEG_INFINITY, |acc, p| acc.max(p.y));
        let yrange = ymax - ymin;
        let ymin = ymin - yrange * 0.1;
        let ymax = ymax + yrange * 0.1;
        let ystep = (ymax - ymin) / (height as f64);

        for point in swarm.points().iter() {
            let x = ((point.x - xmin) / xstep).floor() as usize;
            let y = ((point.y - ymin) / ystep).floor() as usize;
            me.density[x + y * width] += 1.0;
        }

        let max_count = me
            .density
            .iter()
            .fold(0.0, |acc: f64, count| acc.max(*count));
        me.density.iter_mut().for_each(|x| {
            *x /= max_count;
        });

        me
    }
}
