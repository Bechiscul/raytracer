use crate::{
    scene::Scene,
    window::{Color, Framebuffer},
};
use nalgebra::{vector, Vector2, Vector3};

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

pub struct Raytracer<'a> {
    framebuffer: &'a mut Framebuffer,
}

impl<'a> Raytracer<'a> {
    pub fn new(framebuffer: &'a mut Framebuffer) -> Self {
        Self { framebuffer }
    }

    pub fn draw_scene(&mut self, scene: &Scene) {
        let width = self.framebuffer.width();
        let height = self.framebuffer.height();

        self.draw(|pos| {
            Some(Color::from_f32(vector![
                pos[1] as f32 / height as f32,
                pos[0] as f32 / width as f32,
                1.0
            ]))
        });
    }

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
