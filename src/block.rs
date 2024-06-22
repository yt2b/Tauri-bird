use crate::engine::{rect::Rect, renderer::Renderer};
use cgmath::Vector2;
use rand::Rng;

const GROUND_Y: f32 = 600.0;

pub trait Block {
    fn update(&mut self, speed: f32);
    fn is_alive(&self) -> bool;
    fn get_rect(&self) -> &[Rect];
    fn draw(&self, renderer: &Renderer);
}

pub struct Ground {
    rects: Vec<Rect>,
}

impl Ground {
    pub fn new() -> Self {
        Self {
            rects: vec![Rect::new(
                Vector2::new(0.0, GROUND_Y),
                Vector2::new(800.0, 200.0),
            )],
        }
    }
}

impl Block for Ground {
    fn update(&mut self, _: f32) {}

    fn is_alive(&self) -> bool {
        true
    }

    fn get_rect(&self) -> &[Rect] {
        &self.rects
    }

    fn draw(&self, renderer: &Renderer) {
        renderer.set_rgb(116, 80, 48);
        renderer.draw_rect(&self.rects[0]);
    }
}

pub struct Normal {
    rects: Vec<Rect>,
}

impl Normal {
    pub fn new(center_y: f32, len: f32) -> Self {
        let l = len / 2.0;
        let y1 = center_y - l;
        let y2 = center_y + l;
        Self {
            rects: vec![
                Rect::new(Vector2::new(800.0, 0.0), Vector2::new(80.0, y1)),
                Rect::new(Vector2::new(800.0, y2), Vector2::new(80.0, GROUND_Y - y2)),
            ],
        }
    }
}

impl Block for Normal {
    fn update(&mut self, speed: f32) {
        for r in &mut self.rects {
            r.pos.x -= speed;
        }
    }

    fn is_alive(&self) -> bool {
        let rect = &self.rects[0];
        rect.pos.x + rect.size.x >= 0.0
    }

    fn get_rect(&self) -> &[Rect] {
        &self.rects
    }

    fn draw(&self, renderer: &Renderer) {
        renderer.set_rgb(0, 180, 0);
        for r in &self.rects {
            renderer.draw_rect(r);
        }
    }
}

pub struct Slide {
    v_y: f32,
    rects: Vec<Rect>,
}

impl Slide {
    pub fn new(center_y: f32, len: f32, v_y: f32) -> Self {
        let l = len / 2.0;
        let y1 = center_y - l;
        let y2 = center_y + l;
        Self {
            v_y,
            rects: vec![
                Rect::new(Vector2::new(800.0, 0.0), Vector2::new(80.0, y1)),
                Rect::new(Vector2::new(800.0, y2), Vector2::new(80.0, GROUND_Y - y2)),
            ],
        }
    }
}

impl Block for Slide {
    fn update(&mut self, speed: f32) {
        for r in &mut self.rects {
            r.pos.x -= speed;
        }
        self.rects[0].size.y += self.v_y;
        self.rects[1].pos.y += self.v_y;
        self.rects[1].size.y -= self.v_y;
    }

    fn is_alive(&self) -> bool {
        let rect = &self.rects[0];
        rect.pos.x + rect.size.x >= 0.0
    }

    fn get_rect(&self) -> &[Rect] {
        &self.rects
    }

    fn draw(&self, renderer: &Renderer) {
        renderer.set_rgb(0, 180, 0);
        for r in &self.rects {
            renderer.draw_rect(r);
        }
    }
}

pub fn get_block() -> Box<dyn Block> {
    let mut rng = rand::thread_rng();
    let center_y = GROUND_Y / 2.0 + rng.gen_range(-120.0, 120.0);
    if rng.gen_range(0, 2) == 0 {
        Box::new(Normal::new(center_y, 185.0))
    } else {
        let v_y = if center_y < GROUND_Y / 2.0 { 0.5 } else { -0.5 };
        Box::new(Slide::new(center_y, 190.0, v_y))
    }
}
