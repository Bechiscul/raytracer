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

    pub fn update_buffer(&mut self) {
        let width = self.framebuffer.width();
        let height = self.framebuffer.height();

        self.window
            .update_with_buffer(&self.framebuffer, width, height)
            .unwrap()
    }

    pub fn update(&mut self) {
        self.window.update()
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
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
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, pos: Vector2<usize>, color: Color) {
        if let Some(pixel) = self.buffer.get_mut(pos[0] + pos[1] * self.width) {
            *pixel = color.into();
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.buffer = vec![color.into(); self.width * self.height];
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Deref for Framebuffer {
    type Target = Vec<u32>;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Creates a color from a float clamped between [0.0, 1.0].
    ///
    /// # Panics
    ///
    /// Panics if the float is not clamped between [0.0, 1.0].
    pub fn from_f32(color: Vector3<f32>) -> Self {
        let _ = color.map(|component| assert!((0.0..=1.0).contains(&component)));
        unsafe { Self::from_f32_unchecked(color) }
    }

    /// Creates a color from a float clamped between [0.0, 1.0].
    ///
    /// # Safety
    ///
    /// `color` must be clamped to [0.0, 1.0].
    pub unsafe fn from_f32_unchecked(color: Vector3<f32>) -> Self {
        Self {
            r: (color[0] * 256.0) as u8,
            g: (color[1] * 256.0) as u8,
            b: (color[2] * 256.0) as u8,
        }
    }
}

impl From<Vector3<u8>> for Color {
    fn from(color: Vector3<u8>) -> Self {
        Color {
            r: color[0],
            g: color[1],
            b: color[2],
        }
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        let (r, g, b) = (self.r as u32, self.g as u32, self.b as u32);
        r << 16 | g << 8 | b
    }
}
