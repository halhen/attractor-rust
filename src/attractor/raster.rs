use rayon::prelude::*;
use super::swarm::Swarm;
use std::time::{Instant};

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
                        if a.0 < point.x {a.0} else {point.x},
                        if a.1 > point.x {a.1} else {point.x},
                        if a.2 < point.y {a.2} else {point.y},
                        if a.3 > point.y {a.3} else {point.y},
                    )
                })
            .reduce(
                || (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY),
                |a, b| {
                    (
                        if a.0 < b.0 {a.0} else {b.0},
                        if a.1 > b.1 {a.1} else {b.1},
                        if a.2 < b.2 {a.2} else {b.2},
                        if a.3 > b.3 {a.3} else {b.3},
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
        
        let start = Instant::now();
        // This is ~75% of the new() execution time
        for i in indices.iter() {
            me.density[*i] += 1.0;
        }
        println!("{:?}", start.elapsed());

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
