use super::renderer_buffer::*;
use rayon::prelude::*;

pub struct PixelBuffer {
    rbg_summed: Vec<RgbReal>,
    samples_count: usize,
    width: usize,
    height: usize,
}

impl PixelBuffer {
    pub fn new(width: usize, height: usize) -> PixelBuffer {
        PixelBuffer {
            rbg_summed: vec![(0.0, 0.0, 0.0); width * height],
            samples_count: 0,
            width,
            height,
        }
    }
}

impl RendererBuffer for PixelBuffer {
    fn sample_pixels<'a, F: Fn(usize, usize) -> RgbReal + Send + Sync>(&'a mut self, sampler: F) {
        let w = self.width;
        self.rbg_summed
            .par_iter_mut()
            .enumerate()
            .for_each(|(pixel_index, rgb)| {
                let sampled = sampler(pixel_index / w, pixel_index % w);
                rgb.0 += sampled.0;
                rgb.1 += sampled.1;
                rgb.2 += sampled.2;
            });
        self.samples_count += 1;
    }

    fn to_img(&self) -> Vec<u8> {
        let mut img = new_rgbau8_vec(self.width(), self.height());
        if self.samples_count > 0 {
            let sc = self.samples_count as Real;
            img.as_mut_slice()
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, rbga)| {
                    let rgb_sum = &self.rbg_summed[index];
                    rbga.0 = to_channel(rgb_sum.0 / sc);
                    rbga.1 = to_channel(rgb_sum.1 / sc);
                    rbga.2 = to_channel(rgb_sum.2 / sc);
                });
        }
        rgbau8_vec_to_u8_vec(img)
    }

    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }

    fn reset(&mut self) {
        self.samples_count = 0;
        self.rbg_summed.zero_memory();
    }
}
