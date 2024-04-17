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

// fn get_random_sequence() -> [i32; 5] {
//     let mut result = [0; 5];
//     let mut rng = rand::thread_rng();
//     let coin = Uniform::from(0..=1);
//
//     for i in 0..5 {
//         let v = coin.sample(&mut rng);
//         result[i] = v;
//     }
//
//     result
// }

fn get_sequence_from_number(n: i32) -> [i32; 5] {
    let mut result = [0; 5];

    for i in 0..5 {
        result[i] = (n >> i) & 0x1;
    }

    result
}

fn main() {
    let mut sum = 0;
    let mut iter = 0;

    for i in 0..32 {
        for j in 0..32 {
            let x = get_sequence_from_number(i);
            let y = get_sequence_from_number(j);

            sum += lcs(x, y);
            iter += 1;
        }
    }

    // println!("Sums = {sum}");
    println!("E = {}", sum as f64 / iter as f64);
}
