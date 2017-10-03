use std;
use std::cmp;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use node::Node;

type Lattice = Vec<Vec<Node>>;

pub struct SOM {
    lattice: Lattice,
    t: u32,
    total_iterations: u32,

    sigma: f32,
    lambda: f32,
    learn_rate: f32,
}

impl SOM {
    pub fn new<R: Rng>(size: (usize, usize),
                       learn_rate: f32,
                       total_iterations: u32,
                       rng: &mut R) -> Self {
        let between = Range::new(-1f32, 1.);
        let mut lattice = Vec::with_capacity(size.0);

        for _ in 0..size.0 {
            let mut row = Vec::with_capacity(size.1);
            for _ in 0..size.1 {
                row.push(Node(
                        between.ind_sample(rng),
                        between.ind_sample(rng),
                        between.ind_sample(rng),
                        ));
            }

            lattice.push(row);
        }

        let (r,c) = size;
        let map_radius = (cmp::min(r, c) as f32) / 2.0;

        Self {
            lattice,
            learn_rate,
            total_iterations,
            t: 1,
            sigma:  map_radius,
            lambda: (total_iterations as f32) / map_radius.ln(),
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.lattice.len(), self.lattice[0].len())
    }

    pub fn get_lattice(&self) -> &Lattice {
        &self.lattice
    }

    fn bmu(&self, input: &Node) -> (usize, usize) {
        let mut best_dist: f32 = std::f32::INFINITY;
        let mut loc = (0, 0);
        let size = self.get_size();

        for r in 0..size.0 {
            for c in 0..size.1 {
                let dist = input.euc_dist(&self.lattice[r][c]);
                if  dist < best_dist {
                    best_dist = dist;
                    loc = (r, c);
                }
            }
        }

        return loc;
    }

    fn neighborhood(&self, t: u32) -> f32 {
        self.sigma * (-(t as f32) / self.lambda).exp()
    }

    fn learnrate(&self, t: u32) -> f32 {
        self.learn_rate * (-(t as f32) / self.total_iterations as f32).exp()
    }

    fn impact(impact_range: f32, dist: f32) -> f32 {
        (- dist.powf(2.0) / (2.0 * impact_range.powf(2.0))).exp()
    }

    pub fn update(&mut self, input: &Node) {
        let point = self.bmu(input);
        let impact_range = self.neighborhood(self.t);
        let learn_rate   = self.learnrate(self.t);

        let size = self.get_size();

        for r in 0..size.0 {
            for c in 0..size.1 {
                let dist = (((r as i32 - point.0 as i32).pow(2) +
                             (c as i32 - point.1 as i32).pow(2)) as f32).sqrt();
                let node = &mut self.lattice[r][c];
                if dist < impact_range {
                    *node = node.add(&input.sub(node)
                                     .mul(learn_rate)
                                     .mul(SOM::impact(impact_range, dist))
                                     );
                }
            }
        }

        self.t += 1;
    }
}
