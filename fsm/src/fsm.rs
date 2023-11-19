use petgraph::Graph;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct StateMachine {
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub x: i32,
    pub y: i32,
    pub text: String,
    #[serde(rename = "isAcceptState")]
    pub accept_state: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Link {
    #[serde(rename = "Link")]
    NormalLink {
        #[serde(rename = "nodeA")]
        start_node: usize,
        #[serde(rename = "nodeB")]
        end_node: usize,

        text: String,

        #[serde(rename = "lineAngleAdjust")]
        angle_adjust: f64,

        #[serde(rename = "parallelPart")]
        parallel_part: f64,
        #[serde(rename = "perpendicularPart")]
        perpendicular_part: f64,
    },
    StartLink {
        #[serde(rename = "node")]
        node: usize,

        text: String,

        #[serde(rename = "deltaX")]
        delta_x: i32,
        #[serde(rename = "deltaY")]
        delta_y: i32,
    },

    SelfLink {
        #[serde(rename = "node")]
        node: usize,

        text: String,

        #[serde(rename = "anchorAngle")]
        anchor_angle: f64,
    },
}

impl Link {
    /// Get the start and end node of this link.
    ///
    /// If it is a start link, the first node is missing.
    /// If it is a self link, both values are the same.
    pub fn get_nodes(&self) -> (Option<usize>, usize) {
        match self {
            Link::NormalLink {
                start_node,
                end_node,
                ..
            } => (Some(*start_node), *end_node),
            Link::StartLink { node, .. } => (Some(*node), *node),
            Link::SelfLink { node, .. } => (None, *node),
        }
    }

    /// Get the text written on this link.
    pub fn get_text(&self) -> &str {
        match self {
            Link::NormalLink { text, .. } => &text,
            Link::StartLink { text, .. } => &text,
            Link::SelfLink { text, .. } => &text,
        }
    }
}

#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FSMError {
    #[error("this finite state machine contains an infinite loop")]
    InfiniteLoop,

    #[error("this finite state machine has no entry links")]
    NoEntryLinks,

    #[error(
        "a link with this index is pointing at a node with the second index that does not exist"
    )]
    DisjointedLink((usize, usize)),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FSMOutput {
    Accept,
    Reject,
}

impl StateMachine {
    pub fn evaluate(&self, word: String) -> Result<FSMOutput, FSMError> {
        if let Some(err) = self.check_error() {
            return Err(err);
        }
        todo!()
    }

    pub fn check_error(&self) -> Option<FSMError> {
        // Check for existence of entry links
        let has_entry_links = self
            .links
            .iter()
            .any(|l| matches!(l, Link::SelfLink { .. }));
        if !has_entry_links {
            return Some(FSMError::NoEntryLinks);
        }

        // Check for disjointed links
        for (i, l) in self.links.iter().enumerate() {
            let (a, b) = l.get_nodes();
            if b >= self.nodes.len() {
                return Some(FSMError::DisjointedLink((i, b)));
            }
            if a.is_some_and(|a| a >= self.nodes.len()) {
                return Some(FSMError::DisjointedLink((i, a.unwrap())));
            }
        }

        // Check for infinite loops
        // Build a graph from only the links that have zero length.
        let mut graph = Graph::new();
        let nodes: Vec<_> = self.nodes.iter().map(|v| graph.add_node(())).collect();
        for link in self.links.iter().filter(|v| !v.get_text().is_empty()) {
            let (a, b) = link.get_nodes();
            let a = if let Some(v) = a {
                v
            } else {
                continue;
            };
            graph.add_edge(nodes[a], nodes[b], ());
        }
        // If that graph is cyclic, then we have an infinite loop
        if petgraph::algo::is_cyclic_directed(&graph) {
            return Some(FSMError::InfiniteLoop);
        }

        None
    }
}
