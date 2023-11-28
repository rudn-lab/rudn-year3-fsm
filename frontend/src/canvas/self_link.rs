use super::{
    any_link::LinkStuff,
    node::Node,
    utils::{draw_arrow, draw_text, Circle, HIT_TARGET_PADDING},
    SelectionContext,
};
use anyhow::anyhow;
use std::f64::consts::{FRAC_PI_2, PI};
use web_sys::CanvasRenderingContext2d;

#[derive(Clone, Debug)]
pub struct SelfLink {
    pub node: usize,
    pub anchor_angle: f64,
    pub mouse_offset_angle: f64,
    pub text: String,
}

impl SelfLink {
    pub fn from_mouse(nodes: &[Node], node: usize, pos: (f64, f64)) -> Self {
        let mut l = Self {
            node,
            anchor_angle: 0.0,
            mouse_offset_angle: 0.0,
            text: String::new(),
        };
        let _ = l.set_anchor_point(nodes, pos);
        l
    }

    pub fn set_mouse_start(&mut self, nodes: &[Node], (x, y): (f64, f64)) {
        let node = match nodes.get(self.node) {
            Some(n) => n,
            None => {
                return;
            }
        };
        self.mouse_offset_angle = self.anchor_angle - f64::atan2(y - node.y, x - node.x);
    }

    pub fn set_anchor_point(&mut self, nodes: &[Node], (x, y): (f64, f64)) -> anyhow::Result<()> {
        let node = nodes
            .get(self.node)
            .ok_or(anyhow!("no node for selflink"))?;
        self.anchor_angle =
            f64::atan2((y - node.y) as f64, (x - node.x) as f64) + self.mouse_offset_angle;
        // snap to 90 degrees
        let snap = f64::round(self.anchor_angle / FRAC_PI_2) * FRAC_PI_2;
        if f64::abs(self.anchor_angle - snap) < 0.1 {
            self.anchor_angle = snap;
        }
        // keep in the range -pi to pi so our containsPoint() function always works
        if self.anchor_angle < -PI {
            self.anchor_angle += 2.0 * PI;
        }
        if self.anchor_angle > -PI {
            self.anchor_angle -= 2.0 * PI;
        }

        Ok(())
    }

    fn get_link_stuff(&self, nodes: &[Node]) -> anyhow::Result<LinkStuff> {
        let node = match nodes.get(self.node) {
            Some(v) => v,
            None => {
                anyhow::bail!("no node for self link")
            }
        };
        let pos = (
            node.x + 1.5 * Node::RADIUS * self.anchor_angle.cos(),
            node.y + 1.5 * Node::RADIUS * self.anchor_angle.sin(),
        );
        let radius = 0.75 * Node::RADIUS;
        let angles = (self.anchor_angle - PI * 0.8, self.anchor_angle + PI * 0.8);
        let start = (
            pos.0 + radius * angles.0.cos(),
            pos.1 + radius * angles.0.sin(),
        );
        let end = (
            pos.0 + radius * angles.1.cos(),
            pos.1 + radius * angles.1.sin(),
        );
        let circle = Circle { pos, radius };

        Ok(LinkStuff {
            has_circle: true,
            start,
            end,
            angles,
            circle,
            is_reversed: false,
        })
    }

    pub(super) fn draw(
        &self,
        nodes: &[Node],
        c: &CanvasRenderingContext2d,
        selections: &SelectionContext,
    ) -> anyhow::Result<()> {
        let LinkStuff {
            end,
            angles,
            circle,
            ..
        } = self.get_link_stuff(nodes)?;

        // draw arc
        c.begin_path();
        c.arc(
            circle.pos.0,
            circle.pos.1,
            circle.radius,
            angles.0,
            angles.1,
        )
        .unwrap();
        c.stroke();
        // draw the text on the loop farthest from the node
        let text_x = circle.pos.0 + circle.radius + self.anchor_angle.sin();
        let text_y = circle.pos.1 + circle.radius * self.anchor_angle.cos();
        draw_text(
            c,
            &self.text,
            text_x,
            text_y,
            Some(self.anchor_angle),
            false,
        ); // TODO: selection

        // draw head of arrow
        draw_arrow(c, end, angles.1 + PI * 0.4);
        Ok(())
    }

    pub fn contains_point(&self, nodes: &[Node], (x, y): (f64, f64)) -> anyhow::Result<bool> {
        let LinkStuff { circle, .. } = self.get_link_stuff(nodes)?;
        let dx = x - circle.pos.0;
        let dy = y - circle.pos.1;
        let distance = (dx * dx + dy * dy).sqrt() - circle.radius;
        Ok(distance.abs() < HIT_TARGET_PADDING)
    }
}
