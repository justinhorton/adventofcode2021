// Graph code inspired/modified from this example implementation by Nicholas D. Matsakis:
//   http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

use std::collections::HashMap;
use std::fmt::{Error, Formatter};

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<NodeData>,
    pub edges: Vec<EdgeData>,
    pub node_index_by_id: HashMap<String, NodeIndex>,
}

impl Graph {
    pub fn node_index(&self, id: &str) -> Option<NodeIndex> {
        self.node_index_by_id.get(id).copied()
    }

    pub fn add_node(&mut self, id: &str) -> NodeIndex {
        match self.node_index(id) {
            Some(x) => x,
            None => {
                let index = self.nodes.len();
                self.nodes.push(NodeData {
                    id: id.to_string(),
                    first_outgoing_edge: None,
                });
                self.node_index_by_id.insert(id.to_string(), index);
                index
            }
        }
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }

    #[allow(dead_code)]
    pub fn all_successors(&self, source: NodeIndex) -> Vec<NodeIndex> {
        let mut successors = Vec::new();
        self.all_successors_impl(source, &mut successors);
        successors
    }

    fn all_successors_impl(&self, source: NodeIndex, successors: &mut Vec<NodeIndex>) {
        for i in self.successors(source) {
            if !successors.contains(&i) {
                successors.push(i);
                self.all_successors_impl(i, successors);
            }
        }
    }
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

#[derive(Debug)]
pub struct NodeData {
    pub id: String,
    first_outgoing_edge: Option<EdgeIndex>,
}

impl std::fmt::Display for NodeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug)]
pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}
