use std::f32::consts::PI;

use crate::{
    scene::{Camera, Object, Scene, Sphere},
    window::{Color, Framebuffer},
};
use nalgebra::{vector, UnitVector3, Vector2, Vector3};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }
}

pub struct Raytracer<'a> {
    framebuffer: &'a mut Framebuffer,
}

impl<'a> Raytracer<'a> {
    pub fn new(framebuffer: &'a mut Framebuffer) -> Self {
        Self { framebuffer }
    }

    pub fn draw_scene(&mut self, scene: &Scene) {
        let sphere = Sphere {
            center: vector![-3.0, 0.0, -16.0],
            radius: 2.0,
        };

        self.draw(|pixel, (w, h)| {
            let ray = Self::cast(&scene, pixel, w, h);

            // scene.objects().reduce(|accum, item| accum);

            // println!("{:?}", ray);

            // Finds the closest hit.
            let hit = scene
                .objects()
                .filter_map(|object| object.intersect(&ray))
                .reduce(|accum, item| if item.t < accum.t { item } else { accum });

            if let Some(hit) = hit {
                return Some(Color::from_f32(0.5 * hit.normal.add_scalar(1.0)));
            }

            None
        });
    }

    pub fn draw(&mut self, f: impl Fn(Vector2<usize>, (usize, usize)) -> Option<Color>) {
        let width = self.framebuffer.width();
        let height = self.framebuffer.height();

        for y in 0..height {
            for x in 0..width {
                let pos = vector![x, y];
                if let Some(color) = f(pos, (width, height)) {
                    self.framebuffer.set_pixel(pos, color);
                }
            }
        }
    }

    /// Casts a ray from the camera into a pixel on the screen.
    fn cast(scene: &Scene, pixel: Vector2<usize>, w: usize, h: usize) -> Ray {
        let (w, h) = (w as f32, h as f32);
        let Camera { origin, fov } = *scene.camera();
        let aspect_ratio = w / h;

        // Convert fov to radians
        // let fov = fov * (PI / 180.0);

        // TODO(Bech): Forklar algoritme / credit.
        let x = (2.0 * (pixel[0] as f32 + 0.5) / w - 1.0) * (fov / 2.0).tan() * aspect_ratio;
        let y = -(2.0 * (pixel[1] as f32 + 0.5) / h - 1.0) * (fov / 2.0).tan();
        let z = -1.0;

        // It is common convention that the camera faces in the negative z-direction.
        let direction = vector![x, y, z].normalize();
        Ray { origin, direction }
    }
}
