use rand::Rng;
use std::ops::RangeInclusive;

fn main() {
    let stream = 0..=u32::MAX;
    let reservoir = vec![0; 1];
    reservoir_sample(stream, reservoir);
}

fn reservoir_sample(stream: RangeInclusive<u32>, mut reservoir: Vec<u32>) {
    let mut iterator = stream.into_iter();
    let mut i = 0;
    while i < reservoir.len() {
        reservoir[i] = iterator.next().unwrap();
        i += 1;
    }

    let mut change_counter = 0;
    let mut rng = rand::thread_rng();
    while let Some(new_value) = iterator.next() {
        let j = rng.gen_range(0..=i);
        if j < reservoir.len() {
            println!(
                "{i} {change_counter}: {old_value} -> {new_value} {reservoir:?}",
                old_value = reservoir[j]
            );
            reservoir[j] = new_value;
            change_counter += 1;
        }
        i += 1;

        if change_counter >= 50 {
            break;
        }
    }
}
