use std::{collections::{HashMap, HashSet, BinaryHeap}, cmp::Ordering};

type Node = usize;
type Cost = usize;

#[derive(Debug)]
struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list 
                .entry(source)
                .or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
struct Step {
    cost: Cost,
    node: Node,
    history: Vec<Node>,
}

impl Ord for Step { // implementing the Ord trait makes Step comparable to other Step instances

    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }

}


fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    // Need to implement an algorithm similar to Dijkstra shortest path algorithm
    
    let mut to_visit = BinaryHeap::new();

    todo!()
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);
    
    println!("{:?}", g);

    // if let Some((path, cost)) = shortest_path(
    //     &g, 1000, 9000) {
    //         println!("1000->9000, {:?} {}", path, cost);
    // };
}

#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}