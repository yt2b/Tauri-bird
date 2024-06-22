use cgmath::Vector2;

pub struct Rect {
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
}

impl Rect {
    pub fn new(pos: Vector2<f32>, size: Vector2<f32>) -> Self {
        Self { pos, size }
    }

    pub fn is_hit(&self, rect: &Rect) -> bool {
        is_hit(self.pos.x, self.size.x, rect.pos.x, rect.size.x)
            && is_hit(self.pos.y, self.size.y, rect.pos.y, rect.size.y)
    }
}

fn is_hit(s1: f32, l1: f32, s2: f32, l2: f32) -> bool {
    let e1 = s1 + l1;
    let e2 = s2 + l2;
    s1 <= e2 && s2 <= e1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_hit() {
        assert_eq!(is_hit(0.0, 3.0, 3.0, 2.0), true);
        assert_eq!(is_hit(1.0, 3.0, 0.0, 5.0), true);
        assert_eq!(is_hit(4.0, 3.0, 0.0, 5.0), true);
        assert_eq!(is_hit(0.0, 3.0, 4.0, 5.0), false);
        assert_eq!(is_hit(10.0, 8.0, 4.0, 5.0), false);
    }

    #[test]
    fn test_is_hit() {}
}
