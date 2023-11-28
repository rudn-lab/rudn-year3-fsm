use web_sys::CanvasRenderingContext2d;

use super::utils::draw_arrow;

pub struct TemporaryLink {
    pub from: (f64, f64),
    pub to: (f64, f64),
}

impl TemporaryLink {
    pub fn draw(&self, c: &CanvasRenderingContext2d) {
        // draw the line
        c.begin_path();
        c.move_to(self.to.0, self.to.1);
        c.line_to(self.from.0, self.from.1);
        c.stroke();

        // draw the head of the arrow
        draw_arrow(
            c,
            (self.to.0, self.to.1),
            (self.to.1 - self.from.1).atan2(self.to.0 - self.from.0),
        );
    }
}
