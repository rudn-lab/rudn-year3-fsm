use web_sys::CanvasRenderingContext2d;

use super::SelectionContext;

pub(super) fn draw_text(
    c: &CanvasRenderingContext2d,
    real_text: &str,
    mut x: f64,
    mut y: f64,
    angle: Option<f64>,
    me_is_selected: bool,
    me_is_doublesided_link: bool,
    selections: &SelectionContext,
) {
    c.set_font("20px \"Times New Roman\", serif");
    let text = if real_text.len() == 0 && me_is_doublesided_link {
        "(ε)"
    } else {
        real_text
    };
    let width = c.measure_text(text).unwrap().width();

    // center text
    x -= width / 2.0;

    // position the text intelligently if given an angle

    if let Some(angle) = angle {
        let cos = f64::cos(angle);
        let sin = f64::sin(angle);
        let corner_point_x = (width / 2.0 + 5.0) * if cos > 0.0 { 1.0 } else { -1.0 };
        let corner_point_y = (10.0 + 5.0) * if sin > 0.0 { 1.0 } else { -1.0 };
        let slide = sin * f64::powi(sin.abs(), 40) * corner_point_x
            - cos * f64::powi(cos.abs(), 10) * corner_point_y;
        x += corner_point_x - sin * slide;
        y += corner_point_y + cos * slide;
    }

    x = x.round();
    y = y.round();
    c.fill_text(text, x, y + 6.0).unwrap();
    let document_has_focus = gloo::utils::document().has_focus().unwrap();
    if me_is_selected
        && selections.caret_is_displayed
        && document_has_focus
        && selections.canvas_is_focused
    {
        x += width;
        if real_text.len() == 0 {
            x -= width - 2.0;
        }
        c.begin_path();
        c.move_to(x, y - 10.0);
        c.line_to(x, y + 10.0);
        c.stroke();
    }
}

pub fn draw_arrow(c: &CanvasRenderingContext2d, (x, y): (f64, f64), angle: f64) {
    let dx = angle.cos();
    let dy = angle.sin();
    c.begin_path();
    c.move_to(x, y);
    c.line_to(x - 12.0 * dx + 7.0 * dy, y - 12.0 * dy - 7.0 * dx);
    c.line_to(x - 12.0 * dx - 7.0 * dy, y - 12.0 * dy + 7.0 * dx);
    c.fill();
}

pub const SNAP_TO_PADDING: f64 = 6.0; // pixels
pub const HIT_TARGET_PADDING: f64 = 6.0; // pixels

#[derive(Default)]
pub struct Circle {
    pub pos: (f64, f64),
    pub radius: f64,
}

pub fn det(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64) -> f64 {
    a * e * i + b * f * g + c * d * h - a * f * h - b * d * i - c * e * g
}

impl Circle {
    pub fn from_three_points(
        (x1, y1): (f64, f64),
        (x2, y2): (f64, f64),
        (x3, y3): (f64, f64),
    ) -> Self {
        let a = det(x1, y1, 1.0, x2, y2, 1.0, x3, y3, 1.0);
        let bx = -det(
            x1 * x1 + y1 * y1,
            y1,
            1.0,
            x2 * x2 + y2 * y2,
            y2,
            1.0,
            x3 * x3 + y3 * y3,
            y3,
            1.0,
        );
        let by = det(
            x1 * x1 + y1 * y1,
            x1,
            1.0,
            x2 * x2 + y2 * y2,
            x2,
            1.0,
            x3 * x3 + y3 * y3,
            x3,
            1.0,
        );
        let c = -det(
            x1 * x1 + y1 * y1,
            x1,
            y1,
            x2 * x2 + y2 * y2,
            x2,
            y2,
            x3 * x3 + y3 * y3,
            x3,
            y3,
        );
        Circle {
            pos: (-bx / (2.0 * a), -by / (2.0 * a)),
            radius: f64::sqrt(bx * bx + by * by - 4.0 * a * c) / (2.0 * a.abs()),
        }
    }
}
