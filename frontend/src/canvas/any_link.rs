use web_sys::CanvasRenderingContext2d;

use super::{
    node::Node, normal_link::NormalLink, self_link::SelfLink, start_link::StartLink, utils::Circle,
    SelectionContext,
};

#[derive(derive_more::From, Debug, Clone)]
pub enum Link {
    Normal(NormalLink),
    Start(StartLink),
    ToSelf(SelfLink),
}

#[derive(Default)]
pub struct LinkStuff {
    pub has_circle: bool,
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub angles: (f64, f64),
    pub circle: Circle,
    pub is_reversed: bool,
}

impl Link {
    pub(super) fn draw(
        &self,
        nodes: &[Node],
        c: &CanvasRenderingContext2d,
        me_is_selected: bool,
        selections: &SelectionContext,
    ) -> anyhow::Result<()> {
        match self {
            Link::Normal(x) => x.draw(nodes, c, me_is_selected, selections),
            Link::Start(x) => x.draw(nodes, c, me_is_selected, selections),
            Link::ToSelf(x) => x.draw(nodes, c, me_is_selected, selections),
        }
    }

    pub fn contains_point(&self, nodes: &[Node], pos: (f64, f64)) -> anyhow::Result<bool> {
        match self {
            Link::Normal(x) => x.contains_point(nodes, pos),
            Link::Start(x) => x.contains_point(nodes, pos),
            Link::ToSelf(x) => x.contains_point(nodes, pos),
        }
    }

    pub fn set_mouse_start(&mut self, nodes: &[Node], pos: (f64, f64)) {
        match self {
            Link::ToSelf(x) => x.set_mouse_start(nodes, pos),
            _ => {}
        }
    }

    pub fn set_anchor_point(&mut self, nodes: &[Node], pos: (f64, f64)) -> anyhow::Result<()> {
        match self {
            Link::Normal(x) => x.set_anchor_point(nodes, pos),
            Link::Start(x) => x.set_anchor_point(nodes, pos),
            Link::ToSelf(x) => x.set_anchor_point(nodes, pos),
        }
    }
}

impl From<Link> for fsm::fsm::Link {
    fn from(value: Link) -> Self {
        match value {
            Link::Normal(NormalLink {
                node_a,
                node_b,
                text,
                line_angle_adjust,
                parallel_part,
                perpendicular_part,
            }) => Self::NormalLink {
                start_node: node_a,
                end_node: node_b,
                text,
                angle_adjust: line_angle_adjust,
                parallel_part,
                perpendicular_part,
            },
            Link::Start(StartLink { node, text, delta }) => Self::StartLink {
                node,
                text,
                delta_x: delta.0 as i32,
                delta_y: delta.1 as i32,
            },
            Link::ToSelf(SelfLink {
                node,
                text,
                anchor_angle,
                ..
            }) => Self::SelfLink {
                node,
                text,
                anchor_angle,
            },
        }
    }
}

impl From<fsm::fsm::Link> for Link {
    fn from(value: fsm::fsm::Link) -> Self {
        match value {
            fsm::fsm::Link::NormalLink {
                start_node,
                end_node,
                text,
                angle_adjust,
                parallel_part,
                perpendicular_part,
            } => Self::Normal(NormalLink {
                node_a: start_node,
                node_b: end_node,
                text,
                line_angle_adjust: angle_adjust,
                parallel_part,
                perpendicular_part,
            }),
            fsm::fsm::Link::StartLink {
                node,
                text,
                delta_x,
                delta_y,
            } => Self::Start(StartLink {
                node,
                delta: (delta_x as f64, delta_y as f64),
                text,
            }),
            fsm::fsm::Link::SelfLink {
                node,
                text,
                anchor_angle,
            } => Self::ToSelf(SelfLink {
                node,
                anchor_angle,
                mouse_offset_angle: 0.0,
                text,
            }),
        }
    }
}
