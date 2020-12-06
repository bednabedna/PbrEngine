pub use super::super::defs::*;
pub type RgbReal = (Real, Real, Real);
pub type RgbaU8 = (u8, u8, u8, u8);

pub trait Zeroable {
    fn zero_memory(&mut self);
}

impl<T> Zeroable for Vec<T> {
    fn zero_memory(&mut self) {
        unsafe {
            libc::memset(
                self.as_mut_ptr() as _,
                0,
                self.len() * std::mem::size_of::<T>(),
            );
        }
    }
}

pub struct Timer<'a>(std::time::Instant, &'a str);

impl<'a> Timer<'a> {
    pub fn new(id: &'a str) -> Timer {
        Timer(std::time::Instant::now(), id)
    }
    pub fn log(&self) {
        println!("{} {:?}", self.1, self.0.elapsed())
    }
}

pub fn new_rgbau8_vec(width: usize, height: usize) -> Vec<RgbaU8> {
    vec![(0u8, 0u8, 0u8, 255u8); width * height]
}

pub fn rgbau8_vec_to_u8_vec(memory: Vec<RgbaU8>) -> Vec<u8> {
    unsafe {
        let mut memory = std::mem::ManuallyDrop::new(memory);
        let data = std::mem::transmute::<*mut RgbaU8, *mut u8>(memory.as_mut_ptr());
        Vec::from_raw_parts(data, memory.len() * 4, memory.capacity() * 4)
    }
}

pub fn to_channel(mut x: Real) -> u8 {
    debug_assert!(!x.is_nan());
    debug_assert!(x <= 1.0);
    debug_assert!(x >= 0.0);
    if x.is_nan() || x < 0.0 {
        x = 0.0;
    } else if x > 1.0 {
        x = 1.0;
    }
    (255.999 * x.sqrt()) as u8
}

pub trait RendererBuffer {
    fn sample_pixels<'a, F: Fn(usize, usize) -> RgbReal + Send + Sync>(&'a mut self, sampler: F);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn reset(&mut self);
    fn to_img(&self) -> Vec<u8>;
}
