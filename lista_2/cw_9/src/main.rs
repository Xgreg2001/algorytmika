use rand::{distributions::Uniform, Rng};

fn main() {
    let x = generate_random_string(1000);

    println!("INFO: Searching in: {}", x.iter().collect::<String>());

    let matches = find_matching_substrings(x);

    println!("");

    for (i, m) in matches.iter().enumerate() {
        println!("INFO: Match {}: {m}", i + 1);
    }
}

fn generate_random_string(len: usize) -> Vec<char> {
    let alphabet = ['A', 'C', 'G', 'T'];
    let range = Uniform::new(0, 4);
    rand::thread_rng()
        .sample_iter(&range)
        .take(len)
        .map(|i| alphabet[i])
        .collect()
}

fn find_matching_substrings(x: Vec<char>) -> Vec<String> {
    let mut result = vec![];

    let mut seen_atg = false;
    let mut atg_index = 0;

    for i in 0..(x.len() - 2) {
        if x[i] == 'A' && x[i + 1] == 'T' && x[i + 2] == 'G' {
            // println!("DEBUG: Found ATG at: {i}");
            seen_atg = true;
            atg_index = i;
        } else if seen_atg
            && ((x[i] == 'T' && x[i + 1] == 'A' && x[i + 2] == 'A')
                || (x[i] == 'T' && x[i + 1] == 'A' && x[i + 2] == 'G')
                || (x[i] == 'T' && x[i + 1] == 'G' && x[i + 2] == 'A'))
        {
            // println!("DEBUG: Found END at: {i}");
            if i - atg_index > 32 {
                result.push(x[atg_index..=(i + 2)].into_iter().collect());
            }
            seen_atg = false;
        }
    }

    result
}
