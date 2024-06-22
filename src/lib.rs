use bird::Bird;
use engine::game;
use wasm_bindgen::prelude::*;

mod bird;
mod block;
mod engine;
mod player;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async move {
        game::start(Bird::new()).await.expect("Can't start game");
    });
    Ok(())
}
