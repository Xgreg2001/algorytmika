use rand::distributions::{Distribution, Uniform};
use std::cmp::max;

fn lcs(x: [i32; 5], y: [i32; 5]) -> i32 {
    let mut l = [[0; 6]; 6];

    for i in 0..=5 {
        for j in 0..=5 {
            if i == 0 || j == 0 {
                l[i][j] = 0;
            } else if x[i - 1] == y[j - 1] {
                l[i][j] = l[i - 1][j - 1] + 1;
            } else {
                l[i][j] = max(l[i - 1][j], l[i][j - 1]);
            }
        }
    }

    l[5][5]
}

fn get_random_sequence() -> [i32; 5] {
    let mut result = [0; 5];
    let mut rng = rand::thread_rng();
    let coin = Uniform::from(0..=1);

    for i in 0..5 {
        let v = coin.sample(&mut rng);
        result[i] = v;
    }

    result
}

fn main() {
    let tries = 100000000;

    let mut sum = 0;

    for _ in 0..tries {
        let x = get_random_sequence();
        let y = get_random_sequence();

        sum += lcs(x, y)
    }

    println!("E = {}", sum as f64 / tries as f64);
}
