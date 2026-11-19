/// FX-Map regions use left, right, top, bottom coordinates in pattern space.
pub(super) struct Region {
    start: [f32; 2],
    span: [f32; 2],
}

impl Region {
    pub fn new(value: [f32; 4]) -> Result<Self, String> {
        let start = [value[0], value[2]];
        let span = [value[1] - value[0], value[3] - value[2]];
        if value.iter().any(|value| !value.is_finite()) || span.iter().any(|value| !value.is_finite() || value.abs() < f32::EPSILON) {
            return Err("pattern regions must be finite with nonzero width and height".into());
        }

        let mut region = Self { start, span };
        for (start, span) in region.start.iter_mut().zip(&mut region.span) {
            if *span < 0.0 {
                // A reversed interval selects the full axis in pattern space.
                *start = 0.0;
                *span = 1.0;
            }
        }
        Ok(region)
    }

    pub fn point(&self, pixel: [u32; 2], size: [u32; 2]) -> [f32; 2] {
        std::array::from_fn(|axis| self.start[axis] + (pixel[axis] as f32 + 0.5) / size[axis] as f32 * self.span[axis])
    }

    pub fn bounds(&self, center: [f32; 2], radius: [f32; 2], size: [u32; 2]) -> [[u32; 2]; 2] {
        std::array::from_fn(|axis| {
            let a = (center[axis] - radius[axis] - self.start[axis]) / self.span[axis];
            let b = (center[axis] + radius[axis] - self.start[axis]) / self.span[axis];
            [
                (a.min(b) * size[axis] as f32).floor().clamp(0.0, size[axis] as f32) as u32,
                (a.max(b) * size[axis] as f32).ceil().clamp(0.0, size[axis] as f32) as u32,
            ]
        })
    }

    pub fn period(&self) -> [f32; 2] {
        self.span
    }

    pub fn repeats(&self, render: &Self, center: [f32; 2], radius: [f32; 2], axes: [bool; 2]) -> Result<[[i32; 2]; 2], String> {
        let mut result = [[0; 2]; 2];
        for axis in 0..2 {
            if !axes[axis] {
                continue;
            }
            let period = f64::from(self.span[axis]);
            let start = f64::from(self.start[axis]);
            let visible = f64::from(render.start[axis]);
            let center = f64::from(center[axis]);
            let radius = f64::from(radius[axis]);
            // A copy must overlap both windows. Touching an edge alone does not draw it.
            let first = ((start.max(visible) - center - radius) / period).floor() + 1.0;
            let last = (((start + period).min(visible + f64::from(render.span[axis])) - center + radius) / period).ceil() - 1.0;

            // A tiny period or huge pattern cannot create unbounded work, even outside the image
            let coordinates = f64::from(i32::MIN)..=f64::from(i32::MAX);
            if !coordinates.contains(&first) || !coordinates.contains(&last) {
                return Err("pattern repetition exceeds its coordinate budget".into());
            }

            result[axis] = [first as i32, last as i32];
        }

        Ok(result)
    }
}
