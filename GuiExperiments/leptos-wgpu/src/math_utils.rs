#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LinearTransform1D {
    pub m: f32,
    pub c: f32,
}

pub fn get_pixel_to_ndc_transform(size: u32, invert: bool) -> LinearTransform1D {
    // The formula here allows to operate on **pixel centers**. I.e. in a 3x3 canvas,
    // the pixel values 0, 1, 2 map to the three centers of the pixels. The alternative
    // is to use `(x1, x2) = (0.0, size as f32)`, which maps coordinates to **pixel
    // boundaries**.
    // What are the semantics differences when drawing horizontally/vertically aligned lines?
    // - Laterally: In lateral direction, the "pixel center" approach allows to just use
    //   integer values in the lateral component. The "pixel boundary" semantics rather
    //   require to add 0.5 to actually drawn "on" the pixel.
    // - Longitudinally: When drawing a line from "1 to 3" with "pixel center" semantics,
    //   it means that pixel 1 is only filled 50%, pixel 2 is filled 100%, pixel 3 is filled
    //   50%. This means that it is necessary to subtract 0.5 and add 0.5 at the start/end
    //   to draw all 3 pixels with 100% opacity. In contrast, drawing from "1 to 3" with
    //   "pixel boundary" semantics means, that only pixels 1 and 2 are covered 100%, but
    //   pixel 3 isn't covered at all (because the boundaries are start boundaries). In this
    //   case, in order to cover all 3 pixels completely, it is necessary to add +1 to the
    //   end coordinate only.
    //
    // Overall I'm unsure which semantics are preferable. "pixel center" seems a bit simpler
    // to understand regarding the lateral semantics. However, longitudinally it can be a bit
    // awkward (and perhaps a bit more costly) having to add 0.5 to both sides (of course, only
    // if full pixel coverage is desired -- there may be use cases in particular in plotting
    // where the semi-coverage is just fine?). The "pixel boundary" variant could make this
    // a bit easier. However, it may be also a bit surprising when drawing a line from like (1, 1)
    // to (10, 10) that the end pixel (10, 10) is not covered at all?
    //
    // For now let's go for "pixel center" semantics...
    //
    // Note that `(x1, x2) = (0.0, (size - 1) as f32)` doesn't make much sense, because the implied
    // integer grid would simply no longer match with the underlying pixel grid, and all drawing on
    // "nice" coordinates would actually draw across pixels.
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
