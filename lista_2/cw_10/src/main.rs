use rand::distributions::{Distribution, Uniform};
use std::f64::consts::PI;

const N: usize = 100000000;

fn main() {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(0.0, PI);
    let integral: f64 = dist.sample_iter(&mut rng).take(N).map(|v| v.sin()).sum();

    let ans = PI / (N as f64) * integral;

    println!("result = {ans}");
}
