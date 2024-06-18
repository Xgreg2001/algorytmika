use rand::Rng;
use std::ops::RangeInclusive;

fn main() {
    let stream = 0..=u64::MAX;
    let moments = generate_moments(50);
    println!("{:?}", moments);
    reservoir_sample(stream, moments);
}

fn generate_moments(length: usize) -> Vec<usize> {
    let mut moments = vec![0; length];
    moments[0] = 1;

    let mut rng = rand::thread_rng();

    for i in 1..length {
        let prev = moments[i - 1];
        let j: f64 = rng.gen();
        moments[i] = prev + (f64::ceil(j * prev as f64) / (1.0 - j)) as usize
    }

    return moments;
}

fn reservoir_sample(stream: RangeInclusive<u64>, moments: Vec<usize>) {
    let mut iterator = stream.into_iter();
    let mut reservoir = iterator.next().unwrap();
    let mut i = 1;

    let mut moment_counter = 0;
    let mut change_counter = 0;

    while let Some(new_value) = iterator.next() {
        if i == moments[moment_counter] {
            println!("{i} {change_counter}: {reservoir} -> {new_value}",);

            reservoir = new_value;
            moment_counter += 1;
            change_counter += 1;

            if moment_counter >= moments.len() {
                break;
            }
        }

        i += 1;
    }
}
