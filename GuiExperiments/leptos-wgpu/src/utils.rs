#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LinearTransform1D {
    pub m: f32,
    pub c: f32,
}

pub fn get_pixel_to_ndc_transform(size: u32, invert: bool) -> LinearTransform1D {
    let (x1, x2) = (-0.5, size as f32 - 0.5);
    let (y1, y2) = if !invert { (-1.0, 1.0) } else { (1.0, -1.0) };
    let denom = x1 - x2;
    let m = (y1 - y2) / denom;
    let c = (x1 * y2 - x2 * y1) / denom;
    LinearTransform1D { m, c }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    impl LinearTransform1D {
        fn apply(&self, x: f32) -> f32 {
            self.m * x + self.c
        }
    }

    #[test]
    fn test_get_pixel_to_ndc_transform() {
        let transform = get_pixel_to_ndc_transform(3, false);
        assert_eq!(transform.apply(0.0), -2.0 / 3.0);
        assert_eq!(transform.apply(1.0), 0.0);
        assert_eq!(transform.apply(2.0), 2.0 / 3.0);
    }
}
