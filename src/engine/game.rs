use super::browser::{self, LoopClosure};
use super::fps_counter::FPSCounter;
use super::keyboard::{prepare_input, KeyState};
use super::renderer::Renderer;
use anyhow::{anyhow, Result};
use cgmath::Vector2;
use std::{cell::RefCell, rc::Rc};

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;

pub async fn start(mut game: impl Game + 'static) -> Result<()> {
    let mut fps_counter = FPSCounter::new();
    let mut last_frame = browser::now()?;
    let mut accumulated_delta = 0.0;
    let renderer = Renderer::new(browser::context()?);
    let mut key_state = prepare_input()?;
    let closure1: Rc<RefCell<Option<LoopClosure>>> = Rc::new(RefCell::new(None));
    let closure2 = closure1.clone();
    *closure2.borrow_mut() = Some(browser::create_closure(move |perf| {
        key_state.update();
        let frame = perf - last_frame;
        accumulated_delta += frame as f32;
        while accumulated_delta > FRAME_SIZE {
            game.update(&key_state);
            accumulated_delta -= FRAME_SIZE;
        }
        last_frame = perf;
        game.draw(&renderer);
        if cfg!(debug_assertions) {
            fps_counter.add_frame_time(frame);
            draw_fps(&renderer, fps_counter.get_fps());
        }
        browser::request_animation_frame(closure1.borrow().as_ref().unwrap())
            .expect("request_animation_frame failed");
    }));
    browser::request_animation_frame(closure2.borrow().as_ref().ok_or_else(|| anyhow!(""))?)?;
    Ok(())
}

pub trait Game {
    fn update(&mut self, key_state: &KeyState);
    fn draw(&self, renderer: &Renderer);
}

fn draw_fps(renderer: &Renderer, fps: u16) {
    renderer.set_font("16pt sans");
    renderer.set_rgb(255, 255, 255);
    let text = format!("{fps:>2}FPS");
    renderer.draw_text(&text, Vector2::new(0.0, 0.0));
}
