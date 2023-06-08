use web_sys::*;
use super::*;

pub struct Renderer {
    pub context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn clear(&self, rect: &Rect) {
        self.context.clear_rect(rect.x, rect.y, rect.w, rect.h);
    }

    pub fn fill(&self, rect: &Rect, color: &str) {
        self.context.set_fill_style(&JsValue::from_str(&color));
        self.context.fill_rect(rect.x, rect.y, rect.w, rect.h)
    }

    pub fn save(&self) {
        self.context.save();
    }

    pub fn restore(&self) {
        self.context.restore();
    }

    pub fn rotate(&self, angle: f64) {
        self.context.rotate(angle);
    }

    pub fn translate(&self, translate: &FVec) {
        self.context.translate(translate.x, translate.y);
    }

    pub fn line(&self, line: &Line) {
        self.context.move_to(line.start.x, line.start.y);
        self.context.line_to(line.end.x, line.end.y);
        self.context.stroke();
    }

    pub fn text(&self, text: &str, position: FVec) {
        self.context.stroke_text(text, position.x, position.y);
    }

    pub fn stroke_style(&self, style: &str) {
        self.context.set_stroke_style(&JsValue::from_str(style));
    }


    pub fn arc(&self, x: f64, y:f64, radius:f64, start_angle:f64, end_angle:f64, fill: bool) {
        self.context.begin_path();
        self.context.arc(x, y, radius, start_angle, end_angle);
        if fill {
            self.context.fill();
        } else {
            self.context.stroke();
        }
        self.context.close_path();
    }

    pub fn draw_image_with_src_dest(&self, image: &HtmlImageElement, src: &Rect, dest: &Rect) {
        self.context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(image, src.x, src.y, src.w, src.h, dest.x, dest.y, dest.w, dest.h);
    }

    pub fn draw_image_with_dest(&self, image: &HtmlImageElement, src: &Rect) {
        self.context.draw_image_with_html_image_element_and_dw_and_dh(image, src.x, src.y, src.w, src.h);
    }
}

