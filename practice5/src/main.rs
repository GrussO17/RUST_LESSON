
/*
    Here is another classic coding problem. 

    We are going back to Algorithms class!!!! wooooooo

    There are two seperate problems here. I will give you a all the data structures to build a graph. And you will implement a BFS and a DFS search on said graph.

    THe graph is going to be relatively simple.
*/



//The graphs are represented in Node and Edge lists.
pub struct Node {
    name: u32,
}

//edges are bi-directional
pub struct Edge {
    node1: &Node,
    node2: &Node
}

pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}


fn main() {
    let a: Node = {"a"};
    let b: Node = {"b"};
    let c: Node = {"c"};
    let d: Node = {"d"};
    let e: Node = {"e"};
    let graph: Graph = {vec![a, b, c, d, e]; vec![Edge(&a, &b), Edge(&b, &c), Edge(&c, &d), Edge(&d, &e)]};

}

// Perform a Breadth Fist search of the graph. For the easy tests, no loop detection is required.
// Param:   graph -> A graph representation that contains both the start and end nodes.
//          start -> an unmutable pointer to the start node in the graph
//          end   -> an unmutable poitner to the end node in the graph
// Returns: A vector of &Nodes (node pointers), that make up the discovered path.
fn BFS(graph: Graph, start: &Node, end: &Node) -> Vec<&Node> {

}

// Perform a Depth First search of the graph. For the easy tests, no loop detection is required.
// Param:   graph -> A graph representation that contains both the start and end nodes.
//          start -> an unmutable pointer to the start node in the graph
//          end   -> an unmutable poitner to the end node in the graph
// Returns: A vector of &Nodes (node pointers), that make up the discovered path.
fn DFS(graph: Graph, start: &Node, end: &Node) -> Vec<&Node> {

}