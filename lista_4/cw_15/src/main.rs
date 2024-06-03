fn derandomize_sequence(n: usize) -> Vec<i32> {
    let mut x = vec![-1; n as usize];

    // probabity of sucess for a given bit
    let conditional_probability = |i: usize, value: usize| {
        if value == 1 {
            return 0;
        }

        let base: i32 = 2;

        return 1 / (base.pow((n - i - 1) as u32));
    };

    for i in 0..n {
        let prob_0 = conditional_probability(i, 0);
        let prob_1 = conditional_probability(i, 1);

        if prob_0 >= prob_1 {
            x[i] = 0;
        } else {
            x[i] = 1;
        }
    }

    return x;
}

fn main() {
    let n = 20;
    println!("{:?}", derandomize_sequence(n));
}
