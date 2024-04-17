use std::collections::HashSet;

use rand::Rng;

#[derive(Clone)]
struct Edge {
    src: usize,
    dest: usize,
}

struct Graph {
    v: usize,
    e: usize,

    edges: Vec<Edge>,
}

impl Graph {
    fn new(edges: Vec<Edge>) -> Self {
        let out_verts = edges.iter().map(|e| e.src).collect::<HashSet<_>>();
        let in_verts = edges.iter().map(|e| e.dest).collect::<HashSet<_>>();

        let v = in_verts.union(&out_verts).count();
        let e = edges.len();

        Self { v, e, edges }
    }
}

#[derive(Clone)]
struct Subset {
    parent: usize,
    rank: usize,
}

fn karger_min_cut(graph: &mut Graph) -> usize {
    let v_count = graph.v;
    let e_count = graph.e;

    let edge = &mut graph.edges;

    let mut subsets = vec![Subset { parent: 0, rank: 0 }; v_count];

    for v in 0..v_count {
        subsets[v].parent = v;
        subsets[v].rank = 0;
    }

    let mut vertices = v_count;

    while vertices > 2 {
        let i = rand::thread_rng().gen_range(0..e_count);

        let subset1 = find(&mut subsets, edge[i].src);
        let subset2 = find(&mut subsets, edge[i].dest);

        if subset1 == subset2 {
            continue;
        }

        println!("Contracting edge {}-{}", edge[i].src, edge[i].dest);
        vertices -= 1;
        union(&mut subsets, subset1, subset2);
    }

    let mut cutedges = 0;
    for i in 0..e_count {
        let subset1 = find(&mut subsets, edge[i].src);
        let subset2 = find(&mut subsets, edge[i].dest);
        if subset1 != subset2 {
            cutedges += 1;
        }
    }

    cutedges
}

fn find(subsets: &mut Vec<Subset>, i: usize) -> usize {
    if subsets[i].parent != i {
        subsets[i].parent = find(subsets, subsets[i].parent);
    }

    subsets[i].parent
}

fn union(subsets: &mut Vec<Subset>, x: usize, y: usize) {
    let xroot = find(subsets, x);
    let yroot = find(subsets, y);

    if subsets[xroot].rank < subsets[yroot].rank {
        subsets[xroot].parent = yroot;
    } else if subsets[xroot].rank > subsets[yroot].rank {
        subsets[yroot].parent = xroot;
    } else {
        subsets[yroot].parent = xroot;
        subsets[xroot].rank += 1;
    }
}

fn main() {
    let mut edges = vec![];

    // {0<->1,0<->5,0<->8,0<->9,1<->3,1<->6,2<->4,2<->5,3<->6,3<->9,4<->8,4<->9,6<->7,6<->8,7<->9}
    // https://www.wolframcloud.com/env/693f6cd0-ceda-4eb6-be79-645c7c6fb79a

    edges.push(Edge { src: 0, dest: 1 });
    edges.push(Edge { src: 0, dest: 5 });
    edges.push(Edge { src: 0, dest: 8 });
    edges.push(Edge { src: 0, dest: 9 });
    edges.push(Edge { src: 1, dest: 3 });
    edges.push(Edge { src: 1, dest: 6 });
    edges.push(Edge { src: 2, dest: 4 });
    edges.push(Edge { src: 2, dest: 5 });
    edges.push(Edge { src: 3, dest: 6 });
    edges.push(Edge { src: 3, dest: 9 });
    edges.push(Edge { src: 4, dest: 8 });
    edges.push(Edge { src: 4, dest: 9 });
    edges.push(Edge { src: 6, dest: 7 });
    edges.push(Edge { src: 6, dest: 8 });
    edges.push(Edge { src: 7, dest: 9 });

    let mut graph = Graph::new(edges);

    println!(
        "\nCut found by Karger's randomized algo is {}",
        karger_min_cut(&mut graph)
    );
}
