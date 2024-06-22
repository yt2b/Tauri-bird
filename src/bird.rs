use crate::block::{get_block, Block, Ground};
use crate::engine::game::Game;
use crate::engine::keyboard::{Key, KeyState};
use crate::engine::rect::Rect;
use crate::engine::renderer::Renderer;
use crate::player::Player;
use cgmath::Vector2;

const MAX_BLOCK_COUNTER: u32 = 160;
const SPEED: f32 = 3.0;

pub struct Bird {
    score_counter: f32,
    score: u32,
    block_counter: u32,
    is_game_over: bool,
    prev_pressed: bool,
    player: Player,
    blocks: Vec<Box<dyn Block>>,
}

impl Bird {
    pub fn new() -> Self {
        let player = Player::new();
        Self {
            score_counter: 800.0 - player.rect.pos.x,
            score: 0,
            block_counter: MAX_BLOCK_COUNTER,
            is_game_over: false,
            prev_pressed: false,
            player,
            blocks: vec![Box::new(Ground::new()), get_block()],
        }
    }

    fn init(&mut self) {
        self.score_counter = 800.0 - self.player.rect.pos.x;
        self.score = 0;
        self.block_counter = MAX_BLOCK_COUNTER;
        self.is_game_over = false;
        self.prev_pressed = false;
        self.player = Player::new();
        self.blocks = vec![Box::new(Ground::new()), get_block()];
    }

    fn draw_centered_text(&self, renderer: &Renderer, font: &str, text: &str, y: f32) {
        renderer.set_font(font);
        let width = renderer.measure_text(text);
        renderer.draw_text(text, Vector2::new((800.0 - width) / 2.0, y));
    }
}

impl Game for Bird {
    fn update(&mut self, key_state: &KeyState) {
        let pressed = key_state.is_pressed(Key::Space);
        let pressed_once = pressed && !self.prev_pressed;
        if !self.is_game_over {
            if pressed_once {
                self.player.fly();
            }
            self.player.update();
            for block in &mut self.blocks {
                block.update(SPEED);
                for r in block.get_rect() {
                    if self.player.rect.is_hit(r) {
                        self.is_game_over = true;
                    }
                }
            }
            self.blocks.retain(|b| b.is_alive());
            self.block_counter -= 1;
            if self.block_counter == 0 {
                self.block_counter = MAX_BLOCK_COUNTER;
                self.blocks.push(get_block());
            }
            self.score_counter -= SPEED;
            if self.score_counter <= 0.0 {
                self.score += 1;
                self.score_counter = SPEED * MAX_BLOCK_COUNTER as f32;
            }
        } else if pressed_once {
            self.init();
        }
        self.prev_pressed = pressed;
    }

    fn draw(&self, renderer: &Renderer) {
        let r = Rect::new(Vector2::new(0.0, 0.0), Vector2::new(800.0, 600.0));
        renderer.clear(&r);
        renderer.set_rgb(144, 215, 236);
        renderer.draw_rect(&r);
        renderer.set_rgb(255, 0, 0);
        self.player.draw(renderer);
        for block in &self.blocks {
            block.draw(renderer);
        }
        renderer.set_rgb(0, 0, 0);
        renderer.set_font("28pt sans-serif");
        let score = format!("{:0>3}", self.score);
        renderer.draw_text(&score, Vector2::new(5.0, 0.0));
        if self.is_game_over {
            renderer.set_rgb(255, 0, 0);
            self.draw_centered_text(renderer, "50pt sans-serif", "Game Over", 220.0);
            self.draw_centered_text(
                renderer,
                "20pt sans-serif",
                "Press space key to retry",
                300.0,
            );
        }
    }
}
