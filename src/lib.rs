#![deny(missing_docs)]
#![allow(warnings)]

//! Simple iterative solution for PageRank
//! 
//! This is an iterative graph based solution rather than the closed analytical form
//! Graph approaches are nice because the changes needed to make them parallelizable
//! are simper than trying to do distributes matrix calculations
//! 
//! Note here that to do a naive matrix calculation requires to hold the matrix in memory
//! and then perform some sort of SGD based solution
use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;

#[derive(Clone)]
struct GraphNode<T>
where // creating bounds for this struct
    T: Eq + Hash + Clone,
{
    node: T,
    incoming_edges: Vec<usize>,
    outgoing_edges: usize,
    score: f64,
}

/// Pagerank bby
/// note here we are creating a graph with generic types
pub struct Pagerank<T>
where 
    T: Eq + Hash + Clone,
{
    damping: f64,
    nodes: Vec<GraphNode<T>>,
    edges: usize,
    node_positions: HashMap<T, usize>,
    nodes_with_incoming: Option<usize>
}

impl<T> Pagerank<T>
where
    T: Eq + Hash + Clone,
{
    /// Create a new instance
    pub fn new() -> Pagerank<T> {
        Pagerank::<T> {
            damping: 0.85, // magic number for the random surfer
            nodes: Vec::new(),
            edges: 0,
            node_positions: HashMap::<T, usize>::new(),
            nodes_with_incoming: None,
        }
    }
    
    /// setter for the damping factor
    pub fn set_damping_factor(
        &mut self,
        factor: u8,
    ) -> Result<(), String> { 
        if factor >= 100 {
            return Err("{val} needs to be bellow 100".to_string());
        }

        self.damping = factor as f64 / 100_f64;
        Ok(())
    }

    /// BASIC GRAPH STUFF
    
    // Get or create a node
    pub fn get_or_create_node(&mut self, node: T) -> usize {
        match self.node_positions.get(&node) { 
            Some(&value) => value,
            _ => { // if the node doesn't exist, make it
                let id = self.nodes.len();
                self.nodes.push(GraphNode::<T>{
                    node: node.clone(),
                    incoming_edges: Vec::new(),
                    outgoing_edges: 0,
                    score: 1f64 - self.damping
                });
                self.node_positions.insert(node, id);
                self.nodes_with_incoming = None; // new nodes have no edges 
                id // return id
            }
        }
    }

    /// adding nodes to the graph. 
    pub fn add_edge(&mut self, source: T, target: T) {
        let source = self.get_or_create_node(source);
        let target = self.get_or_create_node(target);
        // this is a directed graph
        self.nodes[source].outgoing_edges += 1;
        self.nodes[target].incoming_edges.push(source);
        self.edges +=1;
    }

    /// Get node score
    pub fn get_score(&self, node: T) -> Option<f64> {
        self.node_positions
            .get(&node)
            .map(|id| self.nodes[*id].score)
    }

    /// Get node's number of incoming edges
    pub fn get_incoming_edges(&self, node: T) -> Option<usize> {
        self.node_positions
            .get(&node)
            .map(|id| self.nodes[*id].incoming_edges.len())
    }

    /// Get nodes number of outgoing edges
    pub fn get_outgoing_edges(&self, node: T) -> Option<usize> {
        self.node_positions
            .get(&node)
            .map(|id| self.nodes[*id].outgoing_edges)
    }

    /// len of all edges
    pub fn len_nodes_with_incoming_edges(&mut self) -> usize {
        if let Some(n) = self.nodes_with_incoming {
            return n;
        }

        let mut total = 0 as usize;
        for node in self.nodes.iter() {
            if node.incoming_edges.len() > 0 {
                total += 1;
            }
        }

        self.nodes_with_incoming = Some(total);

        total
    }

    /// 
    pub fn calculate_step(&mut self) -> f64 {
        let mut current_iter = self.nodes.clone();

        let nodes = &self.nodes;

        self.nodes
            .iter()
            .enumerate()
            .map(|(id, n)| {
                // define a closure over the nodes 
                // god fp is rad
                let score = n
                    .incoming_edges
                    .iter()
                    .map(|node| {
                        nodes[*node].score / nodes[*node].outgoing_edges as f64
                    })
                    .sum::<f64>();
                    // 
                    current_iter[id].score = (1f64 - self.damping) + (self.damping * score);
            })
            .for_each(drop); // cleanup

        let convergence: f64 = self
            .nodes
            .iter()
            .enumerate()
            .map(|(id, n) | {
                let diff = n.score - current_iter[id].score;
                diff * diff
            })
            .sum();

        self.nodes = current_iter;
        convergence.sqrt() / self.len_nodes_with_incoming_edges() as f64
    }

    /// calculate pagerank with custom convergence
    pub fn calculate_with_convergence(&mut self, convergence: f64) -> i32 {
        let mut iterations = 0;

        loop {
            if self.calculate_step() < convergence {
                break;
            }
            iterations += 1;
        }
        iterations
    }

    /// Calulate pagerank with predefined covergence
    pub fn calculate(&mut self) -> i32 {
        self.calculate_with_convergence(0.01)
    }
    
    /// Get count of nodes in graph
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Return nodes sorted by pagerank
    pub fn nodes(&self) -> Vec<(&T, f64)> {
        let mut nodes = self
            .nodes
            .iter()
            .map(|node| (&node.node, node.score))
            .collect::<Vec<(&T, f64)>>();
        
        nodes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        nodes
    }

    /// Get count of edges in graph
    pub fn len_node(&self) -> usize {
        self.edges
    }

    /// Is the graph empty?
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

}

impl<T> Default for Pagerank<T>
where
    T: Eq + Hash + Clone 
{
    fn default() -> Self {
        Self::new()
    }    
}


#[cfg(test)]
mod tests {
    /// Yeah im aware i need more test coverage
    use std::ops::Add;

    use crate::Pagerank;

    #[test]
    fn test_set_damping() {
        let mut pagerank = Pagerank::<&str>::new();
        pagerank.set_damping_factor(22);

        assert_eq!(0.22, pagerank.damping)
    }

    #[test]
    fn test_three_nodes_are_created() {
        let mut pagerank = Pagerank::<&str>::new();
        pagerank.add_edge("foo", "bar");
        pagerank.add_edge("foo", "bat");
        assert_eq!(3, pagerank.len())
    }

    //Edge case
    #[test]
    fn test_only_one_node_created() {
        let mut pagerank = Pagerank::<&str>::new();
        pagerank.add_edge("foo", "foo");
        assert_eq!(1, pagerank.len())
    }

    #[test]
    fn test_edges(){
        let mut pagerank = Pagerank::<&str>::new();
        pagerank.add_edge("aaa", "bbb");
        // gonna pack together multiple tests here

        // test one edge
        assert_eq!(Some(1), pagerank.get_incoming_edges("bbb"));
        assert_eq!(Some(0), pagerank.get_incoming_edges("aaa")); // aaa has no incoming edges

        // test two edges
        pagerank.add_edge("ccc", "bbb");
        assert_eq!(Some(2), pagerank.get_incoming_edges("bbb"));
        assert_eq!(Some(0), pagerank.get_outgoing_edges("bbb"))
    }

    #[test]
    fn test_score(){
        let mut pagerank = Pagerank::<&str>::new();
        pagerank.add_edge("aaa", "bbb");
        pagerank.add_edge("bbb", "ccc");
        pagerank.add_edge("ddd", "aaa");
        pagerank.add_edge("eee", "ddd");

        // default score
        assert_eq!(15_i64, (pagerank.get_score("aaa").expect("float") * 100_f64) as i64); 

        // assert default score is shared across nodes
        assert_eq!(pagerank.get_score("aaa"), pagerank.get_score("ddd")); 
    }

    #[test]
    fn test_iteration() {
        let mut pagerank = Pagerank::<&str>::new();
        pagerank.add_edge("aaa", "bbb");
        pagerank.add_edge("bbb", "ccc");
        pagerank.add_edge("ddd", "aaa");
        pagerank.add_edge("eee", "ddd");

        pagerank.calculate_step();
        assert_eq!(
            vec!["aaa", "bbb", "ccc", "ddd", "eee"],
            pagerank.nodes()
                .iter()
                .map(|(node, _)| **node)
                .collect::<Vec<&str>>()
        );
    }

    #[test]
    fn test_full_run() {
        let mut pagerank = Pagerank::<&str>::new();
        pagerank.add_edge("aaa", "bbb");
        pagerank.add_edge("bbb", "aaa");
        pagerank.add_edge("ddd", "aaa");
        pagerank.add_edge("eee", "ddd");

        assert_eq!(16, pagerank.calculate());

        assert_eq!(
            vec!["aaa", "bbb", "ddd", "eee"],
            pagerank.nodes()
                .iter()
                .map(|(node, _)| **node)
                .collect::<Vec<&str>>()
        );
    }
    #[test]
    /// https://en.wikipedia.org/wiki/PageRank#/media/File:PageRanks-Example.svg
    fn test_pagerank_example() {
        let mut pr = Pagerank::new();
        let edges = vec![
            ("D", "A"),
            ("D", "B"),
            ("B", "C"),
            ("C", "B"),
            ("E", "B"),
            ("E", "F"),
            ("F", "B"),
            ("F", "E"),
            ("G", "B"),
            ("G", "E"),
            ("H", "B"),
            ("H", "E"),
            ("I", "B"),
            ("I", "E"),
            ("J", "E"),
            ("K", "E"),
        ];

        edges
            .iter()
            .map(|(l1, l2)| pr.add_edge(*l1, *l2))
            .for_each(drop);

        pr.calculate();

        assert_eq!(
            vec![
                "B", "C", "E", "F", "A", "D", "G", "H", "I", "J", "K"
            ],
            pr.nodes()
                .iter()
                .map(|(node, _)| **node)
                .collect::<Vec<&str>>()
        );
    }  
}
