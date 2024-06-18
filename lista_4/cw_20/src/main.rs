use plotly::{common::Title, Bar, ImageFormat, Layout, Plot};
use rand::Rng;
use std::{
    collections::HashSet,
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

    let words_vec = data.split_whitespace().collect::<Vec<&str>>();
    let words_set = data.split_whitespace().collect::<HashSet<&str>>();

    for seed in seeds {
        let mut results_vec = vec![0; 21];
        let mut results_set = vec![0; 21];

        for word in words_vec.iter() {
            let hash = hash(&mut word.as_bytes(), seed);
            results_vec[hash] += 1;
        }

        for word in words_set.iter() {
            let hash = hash(&mut word.as_bytes(), seed);
            results_set[hash] += 1;
        }

        let x = (0..=20).collect::<Vec<_>>();

        let trace = Bar::new(x.clone(), results_vec);

        let mut plot = Plot::new();
        plot.add_trace(trace);

        let layout = Layout::new().title(Title::new(&format!("murmur3 histogram. seed={seed}")));

        plot.set_layout(layout);

        plot.write_image(format!("hist_{seed}"), ImageFormat::PDF, 1280, 900, 1.0);

        let trace = Bar::new(x, results_set);

        let mut plot = Plot::new();
        plot.add_trace(trace);

        let layout = Layout::new().title(Title::new(&format!(
            "murmur3 histogram unique words. seed={seed}"
        )));

        plot.set_layout(layout);

        plot.write_image(
            format!("hist_unique_{seed}"),
            ImageFormat::PDF,
            1280,
            900,
            1.0,
        );
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
