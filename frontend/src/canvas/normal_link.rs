use std::f64::consts::{FRAC_PI_2, PI};

use anyhow::anyhow;
use web_sys::CanvasRenderingContext2d;

use super::{
    any_link::LinkStuff,
    node::Node,
    utils::{draw_arrow, draw_text, Circle, HIT_TARGET_PADDING, SNAP_TO_PADDING},
    SelectionContext,
};

#[derive(Clone, Debug)]
pub struct NormalLink {
    pub node_a: usize,
    pub node_b: usize,
    pub text: String,
    pub line_angle_adjust: f64,
    pub parallel_part: f64,
    pub perpendicular_part: f64,
}

impl NormalLink {
    pub fn new(node_a: usize, node_b: usize) -> Self {
        Self {
            node_a,
            node_b,
            text: String::new(),
            line_angle_adjust: 0.0,
            parallel_part: 0.5,
            perpendicular_part: 0.0,
        }
    }

    pub fn set_anchor_point(&mut self, nodes: &[Node], (x, y): (f64, f64)) -> anyhow::Result<()> {
        let node_a = nodes.get(self.node_a).ok_or(anyhow!("no node for link"))?;
        let node_b = nodes.get(self.node_b).ok_or(anyhow!("no node for link"))?;
        let dx = node_b.x - node_a.x;
        let dy = node_b.y - node_a.y;
        let scale = f64::sqrt(dx * dx + dy * dy);
        self.parallel_part = ((dx * (x - node_a.x)) + (dy * (y - node_a.y))) / (scale * scale);
        self.perpendicular_part = ((dx * (y - node_a.y)) - dy * (x - node_a.x) as f64) / scale;
        // snap to a straight line
        if self.parallel_part > 0.0
            && self.parallel_part < 1.0
            && f64::abs(self.perpendicular_part) < SNAP_TO_PADDING
        {
            self.line_angle_adjust = PI
                * if self.perpendicular_part < 0.0 {
                    1.0
                } else {
                    0.0
                };
            self.perpendicular_part = 0.0;
        }

        Ok(())
    }

    fn get_link_stuff(&self, nodes: &[Node]) -> anyhow::Result<LinkStuff> {
        let node_a = match nodes.get(self.node_a) {
            Some(v) => v,
            None => {
                anyhow::bail!("missing node for normal link");
            }
        };
        let node_b = match nodes.get(self.node_b) {
            Some(v) => v,
            None => {
                anyhow::bail!("missing node for normal link");
            }
        };

        if self.perpendicular_part == 0.0 {
            let mid = ((node_a.x + node_b.x) / 2.0, (node_a.y + node_b.y) / 2.0);
            let start = node_a.closest_point_on_circle(mid);
            let end = node_b.closest_point_on_circle(mid);
            Ok(LinkStuff {
                has_circle: false,
                start,
                end,
                ..Default::default()
            })
        } else {
            let anchor = {
                let dx = node_b.x - node_a.x;
                let dy = node_b.y - node_a.y;
                let scale = (dx * dx + dy * dy).sqrt();
                (
                    node_a.x + dx * self.parallel_part - dy * self.perpendicular_part / scale,
                    node_a.y + dy * self.parallel_part + dx * self.perpendicular_part / scale,
                )
            };

            let circle =
                Circle::from_three_points((node_a.x, node_a.y), (node_b.x, node_b.y), anchor);

            let is_reversed = self.perpendicular_part > 0.0;
            let reverse_scale = if is_reversed { 1.0 } else { -1.0 };

            let angles = (
                f64::atan2(node_a.y - circle.pos.1, node_a.x - circle.pos.0)
                    - reverse_scale * Node::RADIUS / circle.radius,
                f64::atan2(node_b.y - circle.pos.1, node_b.x - circle.pos.0)
                    + reverse_scale * Node::RADIUS / circle.radius,
            );

            let start = (
                circle.pos.0 + circle.radius * angles.0.cos(),
                circle.pos.1 + circle.radius * angles.0.sin(),
            );
            let end = (
                circle.pos.0 + circle.radius * angles.1.cos(),
                circle.pos.1 + circle.radius * angles.1.sin(),
            );

            Ok(LinkStuff {
                has_circle: true,
                start,
                end,
                angles,
                circle,
                is_reversed,
            })
        }
    }

    pub(super) fn draw(
        &self,
        nodes: &[Node],
        c: &CanvasRenderingContext2d,
        selections: &SelectionContext,
    ) -> anyhow::Result<()> {
        let LinkStuff {
            has_circle,
            start,
            end,
            angles,
            circle,
            is_reversed,
        } = self.get_link_stuff(nodes)?;
        c.begin_path();
        if has_circle {
            c.arc_with_anticlockwise(
                circle.pos.0,
                circle.pos.1,
                circle.radius,
                angles.0,
                angles.1,
                is_reversed,
            )
            .unwrap();
        } else {
            c.move_to(start.0, start.1);
            c.line_to(end.0, end.1);
        }
        let reverse_scale = if is_reversed { 1.0 } else { -1.0 };
        c.stroke();
        // draw arrow head
        if has_circle {
            draw_arrow(c, end, angles.1 - reverse_scale * FRAC_PI_2)
        } else {
            draw_arrow(c, end, f64::atan2(end.1 - start.1, end.0 - start.0));
        }

        // draw text
        if has_circle {
            let (start_angle, mut end_angle) = angles;
            if start_angle < end_angle {
                end_angle += 2.0 * PI;
            }
            let text_angle = (start_angle + end_angle) / 2.0 + if is_reversed { 1.0 } else { 0.0 };
            let pos = (
                circle.pos.0 + circle.radius * text_angle.cos(),
                circle.pos.1 + circle.radius * text_angle.sin(),
            );
            draw_text(c, &self.text, pos.0, pos.1, Some(text_angle), false) // TODO: selection
        }
        Ok(())
    }

    pub fn contains_point(&self, nodes: &[Node], (x, y): (f64, f64)) -> anyhow::Result<bool> {
        let LinkStuff {
            has_circle,
            start,
            end,
            angles,
            circle,
            is_reversed,
        } = self.get_link_stuff(nodes)?;
        if has_circle {
            let dx = x - circle.pos.0;
            let dy = y - circle.pos.1;
            let distance = (dx * dx + dy * dy).sqrt() - circle.radius;
            if distance.abs() < HIT_TARGET_PADDING {
                let mut angle = dy.atan2(dx);
                let mut angles = if is_reversed {
                    (angles.1, angles.0)
                } else {
                    angles
                };

                if angles.1 < angles.0 {
                    angles.1 += PI * 2.0;
                }
                if angle < angles.0 {
                    angle += PI * 2.0;
                } else if angle > angles.1 {
                    angle -= PI * 2.0;
                }

                Ok(angle > angles.0 && angle < angles.1)
            } else {
                Ok(false)
            }
        } else {
            let dx = end.0 - start.0;
            let dy = end.1 - start.1;
            let length = f64::sqrt(dx * dx + dy * dy);
            let percent = (dx * (x - start.0) + dy * (y - start.1)) / (length * length);
            let distance = (dx * (y - start.1) - dy * (x - start.0)) / length;
            Ok(percent > 0.0 && percent < 1.0 && distance.abs() < HIT_TARGET_PADDING)
        }
    }
}
