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
    let mut atga_case = false;

    for i in 0..(x.len() - 2) {
        if x[i] == 'A' && x[i + 1] == 'T' && x[i + 2] == 'G' {
            // println!("DEBUG: Found ATG at: {i}");

            // Handling an edge case where we have ATGA substring
            match x.get(i + 3) {
                Some('A') => {
                    if seen_atg {
                        continue;
                    }
                    atga_case = true;
                }
                _ => (),
            }

            seen_atg = true;
            atg_index = i;
        } else if seen_atg
            && ((x[i] == 'T' && x[i + 1] == 'A' && x[i + 2] == 'A')
                || (x[i] == 'T' && x[i + 1] == 'A' && x[i + 2] == 'G')
                || (x[i] == 'T' && x[i + 1] == 'G' && x[i + 2] == 'A'))
        {
            if atga_case {
                atga_case = false;
                continue;
            }

            // println!("DEBUG: Found END at: {i}");
            if i - atg_index > 32 {
                result.push(x[atg_index..=(i + 2)].into_iter().collect());
            }

            seen_atg = false;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let x = "CGACCGGCCGACTGTAACCGCGGAGTAAATGTTCTTTCCAAGGATGTACCTTTGCAGAAATTTCGCTACCAGCCTCCCACTAACCATACTAGCAGCAGTGAACTTAGAGCATCGACAGGCATCACGATACATAGTGACTCTGGCTCTGCTATTGCCGTGCAACCCGGCAGGAGAGAGCTAATTTGATAAAAAACGTACGAGAGGCGCAAATCTCTCTTTAAGTCGGGGGCCGCTTGATGCTACGGCGGAGCCGGGAGGAATTTCCTCAACCGTCCGAAATACAACCTGCGCCCGTGGCCGAACTTACCAACTCGGCCTGGTGTGTCGAGACACGGCACATGAATAGGACTATGGACTATAACGCCCCTTACCACGCAAACTGGGAGGGTCAAAGTCCGATTTTGGGCGTGGGGGGGGCCGTCCTGCTTTAATGCCCGTACGCATTCAGCCACAGATCTCTTTATCCAGGTTATTCCGGGGCGATCTGGGCTTACACTAAAGCTTCCGTCATTCCAATTGGTTCTGGGGACCCTTTTTGGGGAAACGCGGTTTCGAGACAGCCGCACTCGTTAAAGCTTAAGCCTGTGACCCATACACAATTCTTTCATTACGCGCCAAGCTGGTGCTTCGGGGGTGCACATTTGGCTACAGCTCCCTGTAGAAAGTTTAAGCAACCTCTACATCCGCTGGATTTTAAGGGGTACAGCTCTCAGCTTATGACTTCGATGGGGTGAAAGATAAAAACCTTGACATGCGAAAAGGAGTAAGGCCATCAGTACAACAAACAGATCCAAGATCGGTGATCGGGAAGCGCGCCTAATCGCAGGTATGTACCCGCTTTCTGCACGTACCTTAGGCGCCTCTCGACGCGAACGAGGCTCAGTGCGTCTCGCGATTCACGCGCTTAACTCAGCGTAATGGGCTTCGACCAGAATCCAAGTGTGACGTAAAATCCCAAATCAAGGGAAATCCAGCACGCCACCCTTGCTCCTGGACGGGA".chars().collect();

        let result = find_matching_substrings(x);

        assert_eq!(result, vec!["ATGTACCTTTGCAGAAATTTCGCTACCAGCCTCCCACTAA", "ATGCTACGGCGGAGCCGGGAGGAATTTCCTCAACCGTCCGAAATACAACCTGCGCCCGTGGCCGAACTTACCAACTCGGCCTGGTGTGTCGAGACACGGCACATGA", "ATGCCCGTACGCATTCAGCCACAGATCTCTTTATCCAGGTTATTCCGGGGCGATCTGGGCTTACACTAA"])
    }

    #[test]
    fn atga_case_test() {
        let x = "ATGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAATGA";

        assert!(x.len() >= 36);

        assert_eq!(find_matching_substrings(x.chars().collect()), vec![x]);
    }

    #[test]
    fn random_test() {
        for _ in 0..100 {
            let x = generate_random_string(1000);

            let matches = find_matching_substrings(x);

            for m in matches {
                assert!(m.starts_with("ATG"));
                assert!(m.ends_with("TAA") || m.ends_with("TGA") || m.ends_with("TAG"));
                assert!(m.len() >= 3 + 30 + 3)
            }
        }
    }
}
