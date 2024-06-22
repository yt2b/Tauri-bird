use anyhow::{anyhow, Result};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window found"))
}

pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("No Document found"))
}

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("No Canvas element found"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| anyhow!("Can't convert to HtmlCanvasElement."))
}

pub fn context() -> Result<CanvasRenderingContext2d> {
    canvas()?
        .get_context("2d")
        .map_err(|_| anyhow!("No Context found"))?
        .ok_or_else(|| anyhow!("No Context found"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| anyhow!("Can't convert to CanvasRenderingContext2d"))
}

pub fn now() -> Result<f64> {
    let now = window()?
        .performance()
        .ok_or_else(|| anyhow!("Performance object not found"))?
        .now();
    Ok(now)
}

pub fn request_animation_frame(callback: &Closure<dyn FnMut(f64)>) -> Result<i32> {
    window()?
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .map_err(|_| anyhow!("Cannot request animation frame"))
}

pub type LoopClosure = Closure<dyn FnMut(f64)>;

pub fn create_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
    Closure::wrap(Box::new(f))
}
