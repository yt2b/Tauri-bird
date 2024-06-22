use cgmath::Vector2;
use web_sys::CanvasRenderingContext2d;

use super::rect::Rect;

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn new(context: CanvasRenderingContext2d) -> Self {
        context.set_text_baseline("hanging");
        Self { context }
    }

    pub fn set_rgb(&self, r: u8, g: u8, b: u8) {
        let str = format!("rgb({}, {}, {})", r, g, b);
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(&str));
    }

    pub fn clear(&self, rect: &Rect) {
        self.context.clear_rect(
            rect.pos.x.into(),
            rect.pos.y.into(),
            rect.size.x.into(),
            rect.size.y.into(),
        );
    }

    pub fn draw_rect(&self, rect: &Rect) {
        self.context.fill_rect(
            rect.pos.x.into(),
            rect.pos.y.into(),
            rect.size.x.into(),
            rect.size.y.into(),
        );
    }

    pub fn set_font(&self, text: &str) {
        self.context.set_font(text);
    }

    pub fn draw_text(&self, text: &str, pos: Vector2<f32>) {
        self.context
            .fill_text(text, pos.x.into(), pos.y.into())
            .expect("fill_text failed");
    }

    pub fn measure_text(&self, text: &str) -> f32 {
        let metrics = self
            .context
            .measure_text(text)
            .expect("measure_text failed");
        metrics.width() as f32
    }
}
