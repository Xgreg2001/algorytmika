fn lcs(x: &str, y: &str, z: &str) -> String {
    let x_len = x.len();
    let y_len = y.len();
    let z_len = z.len();
    let mut dp = vec![vec![vec![0; z_len + 1]; y_len + 1]; x_len + 1];
    for i in 1..=x_len {
        for j in 1..=y_len {
            for k in 1..=z_len {
                if x.chars().nth(i - 1) == y.chars().nth(j - 1)
                    && y.chars().nth(j - 1) == z.chars().nth(k - 1)
                {
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                } else {
                    dp[i][j][k] = dp[i - 1][j][k].max(dp[i][j - 1][k]).max(dp[i][j][k - 1]);
                }
            }
        }
    }
    let mut i = x_len;
    let mut j = y_len;
    let mut k = z_len;
    let mut result = String::new();
    while i > 0 && j > 0 && k > 0 {
        if dp[i][j][k] == dp[i - 1][j][k] {
            i -= 1;
        } else if dp[i][j][k] == dp[i][j - 1][k] {
            j -= 1;
        } else if dp[i][j][k] == dp[i][j][k - 1] {
            k -= 1;
        } else {
            result.push(x.chars().nth(i - 1).unwrap());
            i -= 1;
            j -= 1;
            k -= 1;
        }
    }
    result.chars().rev().collect()
}

fn main() {
    println!("Hello, world!");

    let x = "AGGT12";
    let y = "12TXAYB";
    let z = "12XBA";

    let result = lcs(x, y, z);

    println!("LCS is: {}", result);
}
