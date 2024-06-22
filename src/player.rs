use crate::engine::{rect::Rect, renderer::Renderer};
use cgmath::Vector2;

pub struct Player {
    pub rect: Rect,
    velocity: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(Vector2::new(300.0, 150.0), Vector2::new(50.0, 50.0)),
            velocity: 0.0,
        }
    }

    pub fn fly(&mut self) {
        self.velocity = -12.0;
    }

    pub fn update(&mut self) {
        self.velocity += 0.6;
        self.rect.pos.y += self.velocity;
        if self.rect.pos.y <= 0.0 {
            self.rect.pos.y = 0.0;
            self.velocity = 0.0;
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        renderer.set_rgb(235, 145, 0);
        renderer.draw_rect(&self.rect);
    }
}
