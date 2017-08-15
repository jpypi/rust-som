extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

const MAP_DIM: usize  = 32;
const MAP_RADIUS: f32 = (MAP_DIM as f32) / 2.0;
const N_ITERS: u32    = 2000 + 1;
const SIGMA_0: f32    = MAP_RADIUS;
const LAMBDA: f32     = 600.0;//N_ITERS as f32 / MAP_RADIUS.ln();
const LEARN_0: f32    = 0.1;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
struct Point(u32, u32);

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
struct Node(f32, f32, f32);

impl Node {
    fn euc_dist(&self, other: &Node) -> f32 {
        f32::sqrt(
            (self.0 - other.0).powf(2.0) +
            (self.1 - other.1).powf(2.0) +
            (self.2 - other.2).powf(2.0)
            )
    }

    fn mul(&self, x: f32) -> Node {
        Node(
            x * self.0,
            x * self.1,
            x * self.2
            )
    }

    fn sub(&self, b: &Node) -> Node {
        Node(
            self.0 - b.0,
            self.1 - b.1,
            self.2 - b.2
            )
    }

    fn add(&self, b: &Node) -> Node {
        Node(
            self.0 + b.0,
            self.1 + b.1,
            self.2 + b.2
            )
    }
}

fn init<R: Rng>(lattice: &mut [[Node;MAP_DIM];MAP_DIM], rng: &mut R) {
    let between = Range::new(-1f32, 1.);

    for r in 0..MAP_DIM {
        for c in 0..MAP_DIM {
            lattice[r][c] = Node(
                between.ind_sample(rng),
                between.ind_sample(rng),
                between.ind_sample(rng),
                );
        }
    }
}

fn bmu(input: &Node, lattice: &[[Node;MAP_DIM];MAP_DIM]) -> (usize, usize) {
    let mut best_dist: f32 = std::f32::INFINITY;
    let mut loc = (0, 0);

    for r in 0..MAP_DIM {
        for c in 0..MAP_DIM {
            let dist = input.euc_dist(&lattice[r][c]);
            if  dist < best_dist{
                best_dist = dist;
                loc = (r, c);
            }
        }
    }

    return loc;
}

fn neighborhood(t: u32) -> f32 {
    SIGMA_0 * (-(t as f32) / LAMBDA).exp()
}

fn learnrate(t: u32) -> f32 {
    LEARN_0 * (-(t as f32) / N_ITERS as f32).exp()
}

fn impact(impact_range: f32, dist: f32) -> f32 {
    (- dist.powf(2.0) / (2.0 * impact_range.powf(2.0))).exp()
}

fn update(point: (usize, usize),
          input: &Node,
          t: u32,
          lattice: &mut [[Node;MAP_DIM];MAP_DIM]) {

    let impact_range = neighborhood(t);
    let learn_rate   = learnrate(t);

    for r in 0..MAP_DIM {
        for c in 0..MAP_DIM {
            let dist = (((r as i32 - point.0 as i32).pow(2) +
                         (c as i32 - point.1 as i32).pow(2)) as f32).sqrt();
            let node = &mut lattice[r][c];
            if dist < impact_range {
                *node = node.add(&input.sub(node)
                                 .mul(learn_rate)
                                 .mul(impact(impact_range, dist))
                                 );
            }
        }
    }
}

/*
use std::ops;

#[derive(Debug)]
struct AB(i32);

impl<'a, 'b> ops::Sub<&'b AB> for &'a AB {
    type Output = AB;

    fn sub(self, _rhs: &'b AB) -> AB {
        AB(self.0 - _rhs.0)
    }
}

impl ops::Mul<i32> for AB {
    type Output = AB;
    fn mul(self, _rhs: i32) -> AB {
        AB(self.0 * _rhs)
    }
}

impl ops::Mul<AB> for i32 {
    type Output = AB;
    fn mul(self, _rhs: AB) -> AB {
        AB(self * _rhs.0)
    }
}

    let a = AB(4);
    let b = AB(5);
    println!("res: {:?}", &a - &b);
    println!("a: {:?}", 3*a);
*/

fn main() {
    let mut arr = [[Node(0.0,0.0,0.0);MAP_DIM];MAP_DIM];
    let mut rng = rand::thread_rng();

    println!("Initializing...");
    init(&mut arr, &mut rng);

    let samples = vec![
        Node(1.0,0.0,0.0),
        Node(0.0,1.0,0.0),
        Node(0.0,0.0,1.0),
        Node(1.0,1.0,0.0),
        Node(1.0,0.0,1.0),
        Node(0.0,1.0,1.0),
        Node(0.5,0.0,0.0),
        Node(0.5,0.5,0.0),
        Node(1.0,0.647,0.0),
    ];

    let samples_range = Range::new(0, samples.len() as u32);

    println!("Running...");
    for i in 1..N_ITERS {
        let sample_i = samples_range.ind_sample(&mut rng) as usize;
        let sample   = &samples[sample_i];
        let best = bmu(sample, &arr);
        update(best, sample, i, &mut arr);
    }

    println!("Done!");

    println!("{:?}", arr);
}
