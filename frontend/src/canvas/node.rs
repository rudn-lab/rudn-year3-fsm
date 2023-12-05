use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use super::{utils::draw_text, SelectionContext};

#[derive(Clone, Debug)]
pub struct Node {
    pub x: f64,
    pub y: f64,
    pub mouse_offset_x: f64,
    pub mouse_offset_y: f64,
    pub is_accept_state: bool,
    pub text: String,
}

impl Node {
    pub const RADIUS: f64 = 30.0;

    pub fn set_mouse_start(&mut self, (x, y): (f64, f64)) {
        self.mouse_offset_x = self.x - x;
        self.mouse_offset_y = self.y - y;
    }

    pub fn set_anchor_point(&mut self, (x, y): (f64, f64)) {
        self.x = x + self.mouse_offset_x;
        self.y = y + self.mouse_offset_y;
    }

    pub(super) fn draw(
        &self,
        c: &CanvasRenderingContext2d,
        me_is_selected: bool,
        me_is_crossed: bool,
        selections: &SelectionContext,
    ) {
        // Draw the circle
        c.begin_path();
        c.arc(self.x, self.y, Self::RADIUS, 0.0, 2.0 * PI).unwrap();
        c.stroke();

        // Draw the text
        draw_text(
            c,
            &self.text,
            self.x,
            self.y,
            None,
            me_is_selected,
            false,
            selections,
        );

        // draw a double circle for an accept state
        if self.is_accept_state {
            c.begin_path();
            c.arc(self.x, self.y, Self::RADIUS - 6.0, 0.0, 2.0 * PI)
                .unwrap();
            c.stroke();
        }

        if me_is_crossed {
            c.begin_path();
            c.move_to(self.x - Self::RADIUS, self.y);
            c.line_to(self.x + Self::RADIUS, self.y);
            c.stroke();
            c.begin_path();
            c.move_to(self.x, self.y - Self::RADIUS);
            c.line_to(self.x, self.y + Self::RADIUS);
            c.stroke();
        }
    }

    pub fn closest_point_on_circle(&self, (x, y): (f64, f64)) -> (f64, f64) {
        let dx = x - self.x;
        let dy = y - self.y;
        let scale = (dx * dx + dy * dy).sqrt();
        let x = self.x + dx * Self::RADIUS / scale;
        let y = self.y + dy * Self::RADIUS / scale;
        (x, y)
    }

    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        (x - self.x).powi(2) + (y - self.y).powi(2) < Self::RADIUS.powi(2)
    }

    pub fn new(x: f64, y: f64) -> Self {
        Node {
            x,
            y,
            mouse_offset_x: 0.0,
            mouse_offset_y: 0.0,
            is_accept_state: false,
            text: String::new(),
        }
    }
}

impl From<Node> for fsm::fsm::Node {
    fn from(value: Node) -> Self {
        let Node {
            x,
            y,
            is_accept_state,
            text,
            ..
        } = value;
        Self {
            x: x as i32,
            y: y as i32,
            text,
            accept_state: is_accept_state,
        }
    }
}

impl From<fsm::fsm::Node> for Node {
    fn from(value: fsm::fsm::Node) -> Self {
        Self {
            x: value.x as f64,
            y: value.y as f64,
            mouse_offset_x: 0.0,
            mouse_offset_y: 0.0,
            is_accept_state: value.accept_state,
            text: value.text,
        }
    }
}
