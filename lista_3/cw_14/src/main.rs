fn main() {
    let g = Graph {
        vertices: vec![1, 2, 3, 4, 5],
        edges: vec![
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 5),
            (2, 3),
            (2, 4),
            (2, 5),
            (3, 4),
            (3, 5),
            (4, 5),
        ],
    };
    let (a, b) = find_cut(g.clone());
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    let cut_size = cut(&a, &b, &g);
    println!("Cut size: {}", cut_size);

    // max cut: {33,{{1,3,5,7,9,11,12,14,16,17,20},{2,4,6,8,10,13,15,18,19}}}
    let g1 = Graph {
        vertices: vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ],
        edges: vec![
            (1, 4),
            (1, 8),
            (1, 15),
            (2, 9),
            (3, 6),
            (3, 9),
            (3, 10),
            (3, 19),
            (3, 20),
            (4, 7),
            (4, 11),
            (4, 20),
            (5, 6),
            (5, 8),
            (5, 13),
            (5, 19),
            (6, 12),
            (6, 14),
            (6, 16),
            (6, 18),
            (6, 20),
            (7, 8),
            (7, 19),
            (8, 12),
            (8, 16),
            (9, 13),
            (9, 14),
            (9, 15),
            (10, 11),
            (10, 19),
            (10, 20),
            (11, 18),
            (12, 15),
            (12, 19),
            (13, 14),
            (13, 19),
            (13, 20),
            (16, 19),
            (16, 20),
            (17, 19),
        ],
    };

    let (a1, b1) = find_cut(g1.clone());
    println!("A1: {:?}", a1);
    println!("B1: {:?}", b1);

    let cut_size = cut(&a1, &b1, &g1);
    println!("Cut size: {}", cut_size);
}

#[derive(Clone)]
struct Graph {
    vertices: Vec<u32>,
    edges: Vec<(u32, u32)>,
}

/// finds cut that is at least |E|/2
fn find_cut(g: Graph) -> (Vec<u32>, Vec<u32>) {
    let mut a = vec![];
    let mut b = vec![];

    for v in &g.vertices {
        // find if adding v to A gives higher expected value than adding it to B

        let mut a_prime = a.clone();
        let mut b_prime = b.clone();

        a_prime.push(*v);
        b_prime.push(*v);

        let expected_a = calc_expected(&a_prime, &b, &g);
        let expected_b = calc_expected(&a, &b_prime, &g);

        if expected_a > expected_b {
            a = a_prime;
        } else {
            b = b_prime;
        }
    }

    (a, b)
}

fn calc_expected(a: &[u32], b: &[u32], g: &Graph) -> f64 {
    let cut = cut(a, b, g);
    let edges_not_on_boundry = edges_on_boundry(a, b, g);

    return cut + (edges_not_on_boundry / 2.0);
}

/// find amount of edges not connecting a and b
fn edges_on_boundry(a: &[u32], b: &[u32], g: &Graph) -> f64 {
    let mut result = g.edges.len() as u32;

    for e in &g.edges {
        if a.contains(&e.0) && b.contains(&e.1) || a.contains(&e.1) && b.contains(&e.0) {
            result -= 1;
        }
    }

    result as f64
}

fn cut(a: &[u32], b: &[u32], g: &Graph) -> f64 {
    let mut result = 0;

    for e in &g.edges {
        if a.contains(&e.0) && b.contains(&e.1) || a.contains(&e.1) && b.contains(&e.0) {
            result += 1;
        }
    }

    result as f64
}
