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

#[derive(PartialEq, Eq)] // The Eq trait can be derived for those types that have eq defined
struct Step {
    cost: Cost,
    node: Node,
    history: Vec<Node>,
}

impl Ord for Step { // implementing the Ord trait makes Step comparable to other Step instances

    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
        .then_with(|| self.node.cmp(&other.node))
    }

}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cost.partial_cmp(&other.cost) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.node.partial_cmp(&other.node) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.history.partial_cmp(&other.history)
    }
}



fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    // Need to implement an algorithm similar to Dijkstra shortest path algorithm
    
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Step { cost: 0, node: 0, history: vec![] });


    let mut distances: HashMap<Node, Cost> = g.nodes
        .iter()
        .map(|&x| 
            if x == start {
                (x, 0) // the first element represents the final distance, the second element represents the cost of going from start to x
            } else {
                (x, usize::MAX) // The cost of all calculations are given a default value of Infinity, except for the comparison of the starting Node to itself 
        })
        .collect();


        while let Some(Step { cost, node, mut history}) = priority_queue.pop() {
            // Use the while loop to find the distance with the lowest cost 
            if node == goal { // if the current node matches the goal node then return it's history and cost
                history.push(goal);
                return Some((history, cost));
            }

            if let Some(destinations) = g.edges.get(&node) {
                for &(next_dest, next_cost) in destinations.iter() {
                    let mut next_step = { Step {
                        cost: cost + next_cost,
                        node: next_dest,
                        history: history.clone(), // .clone() has to be used here as the Vec<usize> type does not implement the Copy trait. clone() returns a copy of the value
                    } };

                    next_step.history.push(node);
                    priority_queue.push(next_step);

                    if let Some(old_cost) = distances.get_mut(&next_dest) {
                        *old_cost = next_cost
                    }
                }
            }
        }

        None
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);
    
    // println!("{:?}", g);

    if let Some((path, cost)) = shortest_path(
        &g, 1000, 9000) {
            println!("1000->9000, {:?} {}", path, cost);
    };
}

#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}