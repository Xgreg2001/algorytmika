use rand::Rng;
use std::ops::RangeInclusive;

fn main() {
    let stream = 0..=u32::MAX;
    let reservoir = vec![0; 10];
    let moments = generate_moments(100, reservoir.len());
    reservoir_sample(stream, reservoir, moments);
}

fn generate_moments(count: usize, reservoir_size: usize) -> Vec<Moment> {
    let mut moments = vec![];
    let mut counter = 0;
    let mut i = reservoir_size;

    let mut rng = rand::thread_rng();

    while counter < count {
        let j = rng.gen_range(0..=i);
        if j < reservoir_size {
            moments.push(Moment { i, j });
            counter += 1;
        }
        i += 1;
    }

    return moments;
}

struct Moment {
    i: usize,
    j: usize,
}

fn reservoir_sample(stream: RangeInclusive<u32>, mut reservoir: Vec<u32>, moments: Vec<Moment>) {
    let mut iterator = stream.into_iter();
    let mut i = 0;

    while i < reservoir.len() {
        reservoir[i] = iterator.next().unwrap();
        i += 1;
    }

    let mut moment_counter = 0;
    let mut change_counter = 0;
    while let Some(new_value) = iterator.next() {
        if i == moments[moment_counter].i {
            println!(
                "{i} {change_counter}: {old_value} -> {new_value} {reservoir:?}",
                old_value = reservoir[moments[moment_counter].j]
            );
            reservoir[moments[moment_counter].j] = new_value;
            moment_counter += 1;
            change_counter += 1;

            if moment_counter >= moments.len() {
                break;
            }
        }
        i += 1;
    }
}
