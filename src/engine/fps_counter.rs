pub struct FPSCounter {
    fps: u16,
    counter: u16,
    total_frame_time: f64,
}

impl FPSCounter {
    pub fn new() -> Self {
        Self {
            fps: 0,
            counter: 0,
            total_frame_time: 0.0,
        }
    }

    pub fn get_fps(&self) -> u16 {
        self.fps
    }

    pub fn add_frame_time(&mut self, frame_time: f64) {
        self.counter += 1;
        self.total_frame_time += frame_time;
        // 1秒以上経過したらFPSを更新
        if self.total_frame_time >= 1000.0 {
            self.fps = self.counter;
            self.counter = 0;
            self.total_frame_time -= 1000.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fps_counter() {
        let mut counter = FPSCounter::new();
        for _ in 0..60 {
            counter.add_frame_time(16.67);
        }
        assert_eq!(counter.fps, 60);
    }
}
