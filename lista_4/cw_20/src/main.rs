use plotly::{common::Title, Bar, ImageFormat, Layout, Plot};
use rand::Rng;
use std::{
    fs::File,
    io::{BufReader, Read},
};

use murmur3::murmur3_x64_128;

fn hash<T: Read>(data: &mut T, seed: u32) -> usize {
    (murmur3_x64_128(data, seed).unwrap() % 21) as usize
}

fn main() {
    let seeds = [3342, 3849, 2961, 9629, 9764];

    let file = File::open("shakespeare.txt").unwrap();

    let mut reader = BufReader::new(file);

    let mut data = String::new();
    reader.read_to_string(&mut data).unwrap();

    let words = data.split_whitespace();

    for seed in seeds.iter() {
        let mut results = vec![0; 21];

        for word in words.clone() {
            let hash = hash(&mut word.as_bytes(), *seed);
            results[hash] += 1;
        }

        let x = (0..=20).collect::<Vec<_>>();

        let trace = Bar::new(x, results);

        let mut plot = Plot::new();
        plot.add_trace(trace);

        let layout = Layout::new().title(Title::new(&format!("murmur3 histogram. seed={seed}")));

        plot.set_layout(layout);

        plot.write_image(format!("hist_{seed}"), ImageFormat::PDF, 1280, 900, 1.0);
    }

    let a = "William";
    let b = "Shakespeare";

    let mut rng = rand::thread_rng();
    let mut collision_count = 0;

    for _ in 0..1000 {
        let seed = rng.gen();
        let a_hash = hash(&mut a.as_bytes(), seed);
        let b_hash = hash(&mut b.as_bytes(), seed);
        if a_hash == b_hash {
            collision_count += 1;
        }
    }

    println!("Collision count: {}", collision_count);
}
