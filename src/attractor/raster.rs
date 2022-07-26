use rayon::prelude::*;
use super::swarm::Swarm;
//use std::time::Instant;

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

        let (xmin, xmax, ymin, ymax): (f64, f64, f64, f64) = swarm
            .points()
            .par_iter()
            .fold(
                || (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY),
                |a, point| {
                    (
                        a.0.min(point.x),
                        a.1.max(point.x),
                        a.2.min(point.y),
                        a.3.max(point.y)
                    )
                })
            .reduce(
                || (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY),
                |a, b| {
                    (
                        a.0.min(b.0),
                        a.1.max(b.1),
                        a.2.min(b.2),
                        a.3.max(b.3)
                    )
                });

        let xrange = xmax - xmin;
        let xmin = xmin - xrange * 0.1;
        let xmax = xmax + xrange * 0.1;
        let xstep = (xmax - xmin) / (width as f64);
        
        let yrange = ymax - ymin;
        let ymin = ymin - yrange * 0.1;
        let ymax = ymax + yrange * 0.1;
        let ystep = (ymax - ymin) / (height as f64);
        
        let indices: Vec<usize> = swarm
            .points()
            .par_iter()
            .map(|point| {
                let x = ((point.x - xmin) / xstep).floor() as usize;
                let y = ((point.y - ymin) / ystep).floor() as usize;
                x + y * width
            })
            .collect();

        //let start = Instant::now();
        let mut max_count = 0.;
        for i in indices.iter() {
            // This is the most expensive operation in rasterizing.
            // Unchecked access improves throughput ~25%
            unsafe {
                let target = me.density.get_unchecked_mut(*i);
                *target += 1.0;

                // We're mostly limited by CPU cache misses. Might as well do the max calc here too
                if *target > max_count {
                    max_count = *target;
                }
            }
        }
        //println!("{:?}", start.elapsed());

        me.density.iter_mut().for_each(|x| {
            *x /= max_count;
        });

        me
    }
}
