use std::collections::HashSet;

use rand::Rng;

// a structure to represent a unweighted edge in graph
#[derive(Clone)]
struct Edge {
    src: usize,
    dest: usize,
}

// a structure to represent a connected, undirected
// and unweighted graph as a collection of edges.
struct Graph {
    // V-> Number of vertices, E-> Number of edges
    v: usize,
    e: usize,

    // graph is represented as an Vec of edges.
    // Since the graph is undirected, the edge
    // from src to dest is also edge from dest
    // to src. Both are counted as 1 edge here.
    edges: Vec<Edge>,
}

impl Graph {
    fn new(edges: Vec<Edge>) -> Self {
        let out_verts = edges.iter().map(|e| e.src).collect::<HashSet<_>>();
        let in_verts = edges.iter().map(|e| e.dest).collect::<HashSet<_>>();

        let v = in_verts.union(&out_verts).count();
        // asuming no duplicates
        let e = edges.len();

        Self { v, e, edges }
    }
}

// A structure to represent a subset for union-find
#[derive(Clone)]
struct Subset {
    parent: usize,
    rank: usize,
}

// A very basic implementation of Karger's randomized
// algorithm for finding the minimum cut. Please note
// that Karger's algorithm is a Monte Carlo Randomized algo
// and the cut returned by the algorithm may not be
// minimum always
fn karger_min_cut(graph: &mut Graph) -> usize {
    // Get data of given graph
    let v_count = graph.v;
    let e_count = graph.e;

    let edge = &mut graph.edges;

    // Allocate memory for creating V subsets.
    let mut subsets = vec![Subset { parent: 0, rank: 0 }; v_count];

    // Create V subsets with single elements
    for v in 0..v_count {
        subsets[v].parent = v;
        subsets[v].rank = 0;
    }

    // Initially there are V vertices in
    // contracted graph
    let mut vertices = v_count;

    // Keep contracting vertices until there are
    // 2 vertices.
    while vertices > 2 {
        let i = rand::thread_rng().gen_range(0..e_count);

        // Find vertices (or sets) of two corners
        // of current edge
        let subset1 = find(&mut subsets, edge[i].src);
        let subset2 = find(&mut subsets, edge[i].dest);

        // If two corners belong to same subset,
        // then no point considering this edge
        if subset1 == subset2 {
            continue;
        }

        println!("Contracting edge {}-{}", edge[i].src, edge[i].dest);
        vertices -= 1;
        union(&mut subsets, subset1, subset2);
    }

    // Now we have two vertices (or subsets) left in
    // the contracted graph, so count the edges between
    // two components and return the count.
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

// A utility function to find set of an element i
// (uses path compression technique)
fn find(subsets: &mut Vec<Subset>, i: usize) -> usize {
    // find root and make root as parent of i
    // (path compression)
    if subsets[i].parent != i {
        subsets[i].parent = find(subsets, subsets[i].parent);
    }

    subsets[i].parent
}

// A function that does union of two sets of x and y
// (uses union by rank)
fn union(subsets: &mut Vec<Subset>, x: usize, y: usize) {
    let xroot = find(subsets, x);
    let yroot = find(subsets, y);

    // Attach smaller rank tree under root of high
    // rank tree (Union by Rank)
    if subsets[xroot].rank < subsets[yroot].rank {
        subsets[xroot].parent = yroot;
    } else if subsets[xroot].rank > subsets[yroot].rank {
        subsets[yroot].parent = xroot;
    } else {
        // If ranks are same, then make one as root and
        // increment its rank by one
        subsets[yroot].parent = xroot;
        subsets[xroot].rank += 1;
    }
}

// Creates a graph with V vertices and E edges
fn create_graph(v: usize, e: usize) -> Graph {
    Graph {
        v,
        e,
        edges: vec![Edge { src: 0, dest: 0 }; e],
    }
}

// Driver program to test above functions
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
