use std::ops::Deref;

use nalgebra::{Vector2, Vector3};

// use minifb::{self, WindowOptions};

pub struct Window {
    window: minifb::Window,
    framebuffer: Framebuffer,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        use minifb::{Window, WindowOptions};
        let window = Window::new(title, width, height, WindowOptions::default()).unwrap();
        let framebuffer = Framebuffer::new(width, height);
        Self {
            window,
            framebuffer,
        }
    }

    pub fn framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }

    pub fn framebuffer_mut(&mut self) -> &mut Framebuffer {
        &mut self.framebuffer
    }

    pub fn update_w(&mut self) {
        let width = self.framebuffer.width();
        let height = self.framebuffer.height();

        self.window
            .update_with_buffer(&self.framebuffer, width, height)
            .unwrap()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Framebuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self::with_color(width, height, Vector3::<f32>::default())
    }

    pub fn with_color(width: usize, height: usize, color: Vector3<f32>) -> Self {
        Self {
            buffer: { vec![Self::from_vec3_rgb(color); width * height] },
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, pos: Vector2<usize>, color: Vector3<f32>) {
        if let Some(pixel) = self.buffer.get_mut(pos[0] + pos[1] * self.width) {
            *pixel = Self::from_vec3_rgb(color);
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn from_vec3_rgb(color: Vector3<f32>) -> u32 {
        let r = (color[0] * 255.0).round() as u32;
        let g = (color[1] * 255.0).round() as u32;
        let b = (color[2] * 255.0).round() as u32;
        (r << 16) | (g << 8) | b
    }
}

impl Deref for Framebuffer {
    type Target = Vec<u32>;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
