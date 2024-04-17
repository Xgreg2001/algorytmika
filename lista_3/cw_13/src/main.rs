use core::f64;

use rand::distributions::{Distribution, Uniform};

fn integral(n: usize) -> f64 {
    let f = |x: f64| f64::sqrt(1.0 - x * x);
    let dist = Uniform::new_inclusive(0.0, 1.0);
    let sum: f64 = dist
        .sample_iter(rand::thread_rng())
        .take(n)
        .map(|x| f(x))
        .sum();
    return sum / n as f64;
}

fn stratified_sampling(n: usize) -> f64 {
    let f = |x: f64| f64::sqrt(1.0 - x * x);

    let samples = n / 2;

    let dist_1 = Uniform::new_inclusive(0.0, 0.5);
    let dist_2 = Uniform::new_inclusive(0.5, 1.0);

    let sum_1: f64 = dist_1
        .sample_iter(rand::thread_rng())
        .take(samples)
        .map(|x| f(x))
        .sum();

    let sum_2: f64 = dist_2
        .sample_iter(rand::thread_rng())
        .take(samples)
        .map(|x| f(x))
        .sum();

    let mean_1 = sum_1 / samples as f64;
    let mean_2 = sum_2 / samples as f64;

    return 0.5 * (mean_1 + mean_2);
}

fn antitetic_variaty(n: usize) -> f64 {
    let f = |x: f64| f64::sqrt(1.0 - x * x);

    let samples = n / 2;

    let dist = Uniform::new_inclusive(0.0, 1.0);

    let sum: f64 = dist
        .sample_iter(rand::thread_rng())
        .take(samples)
        .map(|x| f(x) + f(1.0 - x))
        .sum();

    return sum / n as f64;
}

fn combined(n: usize) -> f64 {
    let f = |x: f64| f64::sqrt(1.0 - x * x);
    let samples = n / 2;

    let dist_1 = Uniform::new_inclusive(0.0, 0.5);
    let dist_2 = Uniform::new_inclusive(0.5, 1.0);

    let sum_1: f64 = dist_1
        .sample_iter(rand::thread_rng())
        .take(samples / 2)
        .map(|x| f(x) + f(0.5 - x))
        .sum();

    let sum_2: f64 = dist_2
        .sample_iter(rand::thread_rng())
        .take(samples / 2)
        .map(|x| f(x) + f(1.5 - x))
        .sum();

    let mean_1 = sum_1 / samples as f64;
    let mean_2 = sum_2 / samples as f64;

    return 0.5 * (mean_1 + mean_2);
}

fn main() {
    let correct_value = f64::consts::PI / 4.0;

    println!("n,integral_error,stratified_sampling_error,antitetic_variaty_error,combined_error");

    for n in (1000000..=100000000).step_by(5000000) {
        let integral_value = integral(n);
        let stratified_sampling_value = stratified_sampling(n);
        let antitetic_variaty_value = antitetic_variaty(n);
        let combined_value = combined(n);
        let integral_error = (correct_value - integral_value).abs();
        let stratified_sampling_error = (correct_value - stratified_sampling_value).abs();
        let antitetic_variaty_error = (correct_value - antitetic_variaty_value).abs();
        let combined_error = (correct_value - combined_value).abs();

        println!(
            "{},{},{},{},{}",
            n, integral_error, stratified_sampling_error, antitetic_variaty_error, combined_error
        );
    }
}
