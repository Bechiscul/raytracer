use nalgebra::{Vector2, Vector3};
use std::{
    fs::OpenOptions,
    ops::{Deref, DerefMut},
};

pub struct Framebuffer {
    pub buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, pos: Vector2<usize>, color: Vector3<f32>) {
        self.buffer[pos[0] + pos[1] * self.width] = Self::color_from_vec3_rgb(color);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn color_from_vec3_rgb(color: Vector3<f32>) -> u32 {
        let r = (color[0].min(1.0) * 255.0).round() as u32;
        let g = (color[1].min(1.0) * 255.0).round() as u32;
        let b = (color[2].min(1.0) * 255.0).round() as u32;
        (r << 16) | (g << 8) | b
    }
}

impl Deref for Framebuffer {
    type Target = Vec<u32>;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for Framebuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
