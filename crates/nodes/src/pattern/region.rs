/// FX-Map regions use left, right, top, bottom coordinates in pattern space.
pub(super) struct Region {
    start: [f32; 2],
    end: [f32; 2],
    span: [f32; 2],
}

impl Region {
    pub fn new(value: [f32; 4]) -> Result<Self, String> {
        let mut start = [value[0], value[2]];
        let mut end = [value[1], value[3]];
        let mut span = [end[0] - start[0], end[1] - start[1]];
        if value.iter().any(|value| !value.is_finite()) || span.iter().any(|value| !value.is_finite() || value.abs() < f32::EPSILON) {
            return Err("pattern regions must be finite with nonzero width and height".into());
        }

        for axis in 0..2 {
            if span[axis] < 0.0 {
                // A reversed interval selects the full axis in pattern space.
                start[axis] = 0.0;
                end[axis] = 1.0;
                span[axis] = 1.0;
            }
        }
        Ok(Self { start, end, span })
    }

    pub fn contains(&self, point: [f32; 2]) -> bool {
        point.into_iter().enumerate().all(|(axis, value)| (self.start[axis]..self.end[axis]).contains(&value))
    }

    pub fn point(&self, pixel: [u32; 2], size: [u32; 2]) -> [f32; 2] {
        std::array::from_fn(|axis| self.pixel_center(axis, pixel[axis], size[axis]))
    }

    pub fn pixel_center(&self, axis: usize, pixel: u32, size: u32) -> f32 {
        self.start[axis] + (pixel as f32 + 0.5) / size as f32 * self.span[axis]
    }

    pub fn vertex_point(&self, pixel: [f64; 2], dimensions: [u32; 2]) -> [f32; 2] {
        std::array::from_fn(|i| self.start[i] + pixel[i] as f32 / dimensions[i] as f32 * self.span[i])
    }

    pub fn stamp_vertices(&self, axis: usize, center: f32, radius: f32, pixels: u32) -> [f32; 2] {
        let scale = pixels as f32 / self.span[axis];
        // Store the two triangle vertices in pattern space before projecting them into the render window.
        [center - radius, center + 3.0 * radius].map(|v| (v - self.start[axis]) * scale)
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

    pub fn copy_origin(&self, center: [f32; 2], axes: [bool; 2]) -> [f32; 2] {
        std::array::from_fn(|i| {
            let start = f64::from(self.start[i]);
            let offset = f64::from(center[i]) - start;
            let period = f64::from(self.span[i]);
            // Adjacent copy indices cease to be distinct F32 translations at 2^24.
            if axes[i] && offset.abs() >= 16_777_216.0 * period { (start + offset.rem_euclid(period)) as f32 } else { center[i] }
        })
    }

    pub fn copy_centers(&self, axis: usize, center: f32, [first, last]: [i32; 2]) -> impl Iterator<Item = f32> + Clone {
        let period = self.span[axis];
        // Each step starts from the preceding F32 center, including across fractional periods.
        let outer = center + last as f32 * period;
        (first..=last).rev().scan(outer, move |next, _| {
            let current = *next;
            *next -= period;
            Some(current)
        })
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
            let last = ((f64::from(self.end[axis].min(render.end[axis])) - center + radius) / period).ceil() - 1.0;

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
