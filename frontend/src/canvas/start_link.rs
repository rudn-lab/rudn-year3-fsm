use super::{
    node::Node,
    utils::{draw_arrow, draw_text, HIT_TARGET_PADDING, SNAP_TO_PADDING},
    SelectionContext,
};
use anyhow::anyhow;
use web_sys::CanvasRenderingContext2d;

#[derive(Clone, Debug)]
pub struct StartLink {
    pub node: usize,
    pub delta: (f64, f64),
    pub text: String,
}

impl StartLink {
    pub fn new(node: usize) -> Self {
        Self {
            node,
            delta: (0.0, 0.0),
            text: String::new(),
        }
    }

    pub fn set_anchor_point(&mut self, nodes: &[Node], (x, y): (f64, f64)) -> anyhow::Result<()> {
        self.delta.0 = x - nodes
            .get(self.node)
            .ok_or(anyhow!("no node for startlink"))?
            .x;
        self.delta.1 = y - nodes
            .get(self.node)
            .ok_or(anyhow!("no node for startlink"))?
            .y;

        if self.delta.0.abs() < SNAP_TO_PADDING {
            self.delta.0 = 0.0;
        }
        if self.delta.1.abs() < SNAP_TO_PADDING {
            self.delta.1 = 0.0;
        }
        Ok(())
    }

    pub fn get_start_link_endpoints(
        &self,
        nodes: &[Node],
    ) -> anyhow::Result<((f64, f64), (f64, f64))> {
        let node = match nodes.get(self.node) {
            Some(n) => n,
            None => {
                anyhow::bail!("no node for start link");
            }
        };
        let start_x = node.x + self.delta.0;
        let start_y = node.y + self.delta.1;
        let end = node.closest_point_on_circle((start_x, start_y));
        Ok(((start_x, start_y), end))
    }

    pub(super) fn draw(
        &self,
        nodes: &[Node],
        c: &CanvasRenderingContext2d,
        selections: &SelectionContext,
    ) -> anyhow::Result<()> {
        let (start, end) = self.get_start_link_endpoints(nodes)?;
        c.begin_path();
        c.move_to(start.0, start.1);
        c.line_to(end.0, end.1);
        c.stroke();

        // draw the text at the end without the arrow
        let text_angle = (start.1 - end.1).atan2(start.0 - end.0);
        draw_text(c, &self.text, start.0, start.1, Some(text_angle), false); // TODO: selection;

        // draw the head of the arrow
        draw_arrow(c, end, f64::atan2(-self.delta.1, -self.delta.0));
        Ok(())
    }

    pub fn contains_point(&self, nodes: &[Node], (x, y): (f64, f64)) -> anyhow::Result<bool> {
        let (start, end) = self.get_start_link_endpoints(nodes)?;
        let dx = end.0 - start.0;
        let dy = end.1 - start.1;
        let length = f64::sqrt(dx * dx + dy * dy);
        let percent = (dx * (x - start.0) + dy * (y - start.1)) / (length * length);
        let distance = (dx * (y - start.1) - dy * (x - start.0)) / length;
        Ok(percent > 0.0 && percent < 1.0 && distance.abs() < HIT_TARGET_PADDING)
    }
}
