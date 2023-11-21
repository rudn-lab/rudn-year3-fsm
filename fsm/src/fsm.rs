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
            Link::StartLink { node, .. } => (None, *node),
            Link::SelfLink { node, .. } => (Some(*node), *node),
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FSMOutput {
    Accept,
    Reject,
}

impl From<FSMOutput> for bool {
    fn from(value: FSMOutput) -> Self {
        match value {
            FSMOutput::Accept => true,
            FSMOutput::Reject => false,
        }
    }
}

impl StateMachine {
    pub fn evaluate(&self, word: &str) -> Result<FSMOutput, FSMError> {
        if let Some(err) = self.check_error() {
            return Err(err);
        }
        self.evaluate_unchecked(word)
    }

    pub fn evaluate_unchecked(&self, word: &str) -> Result<FSMOutput, FSMError> {
        #[derive(Debug)]
        struct Cursor<'a> {
            remaining_word: &'a str,
            node_idx: usize,
        }
        let mut cursors = vec![];
        let start_links: Vec<_> = self
            .links
            .iter()
            .filter(|v| matches!(v, Link::StartLink { .. }))
            .collect();
        if start_links.is_empty() {
            return Err(FSMError::NoEntryLinks);
        }

        for link in start_links {
            if let Some(after_prefix) = word.strip_prefix(link.get_text()) {
                cursors.push(Cursor {
                    remaining_word: after_prefix,
                    node_idx: link.get_nodes().1,
                });
            }
        }

        // TODO: better termination criteria
        let mut deadline = 100_00usize;

        loop {
            if deadline == 0 {
                log::error!("Runtime check failed: too many steps needed for FSM");
                log::error!("FSM: {self:?}");
                return Err(FSMError::InfiniteLoop);
            }
            deadline -= 1;

            let mut new_cursors = vec![];

            // If there are cursors with empty words, check if they're at accepting nodes.
            // If they are, the machine accepts.
            for cursor in cursors.iter() {
                if cursor.remaining_word.is_empty() {
                    if self.nodes[cursor.node_idx].accept_state {
                        return Ok(FSMOutput::Accept);
                    }
                }
            }

            // Remove all cursors with empty words that ended up on non-accepting nodes.
            cursors.retain(|v| !v.remaining_word.is_empty());

            // If there are no cursors left, the machine rejects.
            if cursors.is_empty() {
                return Ok(FSMOutput::Reject);
            }

            for Cursor {
                remaining_word,
                node_idx,
            } in cursors.iter()
            {
                for link in self
                    .links
                    .iter()
                    .filter(|v| v.get_nodes().0 == Some(*node_idx))
                {
                    log::debug!("Cursor is at {remaining_word:?} at {node_idx}: link {link:?}");
                    if let Some(after_prefix) = remaining_word.strip_prefix(link.get_text()) {
                        log::debug!("After prefix: {after_prefix:?}");
                        new_cursors.push(Cursor {
                            remaining_word: after_prefix,
                            node_idx: link.get_nodes().1,
                        })
                    }
                }
            }

            cursors.clear();
            cursors.extend(new_cursors.into_iter());
            log::debug!("{cursors:?}");
        }
    }

    pub fn check_error(&self) -> Option<FSMError> {
        // Check for existence of entry links
        let has_entry_links = self
            .links
            .iter()
            .any(|l| matches!(l, Link::StartLink { .. }));
        if !has_entry_links {
            log::info!("Pre-check failed: FSM has no start link");
            return Some(FSMError::NoEntryLinks);
        }

        // Check for disjointed links
        for (i, l) in self.links.iter().enumerate() {
            let (a, b) = l.get_nodes();
            if b >= self.nodes.len() {
                log::info!("Pre-check failed: FSM has link to node {b}, which does not exist");
                return Some(FSMError::DisjointedLink((i, b)));
            }
            if let Some(a) = a {
                if a >= self.nodes.len() {
                    log::info!(
                        "Pre-check failed: FSM has link from node {a:?}, which does not exist"
                    );
                    return Some(FSMError::DisjointedLink((i, a)));
                }
            }
        }

        // Check for infinite loops
        // Build a graph from only the links that have zero length.
        let mut graph = Graph::new();
        let nodes: Vec<_> = self.nodes.iter().map(|_v| graph.add_node(())).collect();
        for link in self
            .links
            .iter()
            .filter(|v| v.get_text().is_empty())
            .filter(|v| !matches!(v, Link::StartLink { .. }))
        {
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
            log::info!("Pre-check failed: FSM has loop made out of zero-length links");
            log::debug!("Graph: {graph:?}");
            return Some(FSMError::InfiniteLoop);
        }

        None
    }
}
