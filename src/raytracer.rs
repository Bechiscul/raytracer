use crate::{
    scene::Scene,
    window::{Color, Framebuffer},
};
use nalgebra::{vector, Vector2, Vector3};

pub struct Ray {
    origo: Vector3<usize>,
    direction: Vector3<f32>,
}

pub struct Raytracer<'a> {
    framebuffer: &'a mut Framebuffer,
}

impl<'a> Raytracer<'a> {
    pub fn new(framebuffer: &'a mut Framebuffer) -> Self {
        Self { framebuffer }
    }

    pub fn draw_scene(scene: &Scene) {}

    pub fn draw(&mut self, f: impl Fn(Vector2<usize>) -> Option<Color>) {
        for y in 0..self.framebuffer.height() {
            for x in 0..self.framebuffer.width() {
                let pos = vector![x, y];
                if let Some(color) = f(pos) {
                    self.framebuffer.set_pixel(pos, color);
                }
            }
        }
    }
}
