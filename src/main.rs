use framebuffer::*;
use minifb::*;
use nalgebra::vector;
use raytracer::*;
use scene::*;

mod framebuffer;
mod raytracer;
mod scene;

const WIDTH: usize = 1000;
const HEIGHT: usize = (WIDTH as f32 / (16.0 / 9.0)) as usize;

// Optimization techniques:
// - Multithreading
// - Sort-by-depth algorithm
// - Tagged-unions.

fn example_scene() -> Scene {
    let mut scene = Scene::new();

    let ivory = Material {
        diffuse: vector![0.4, 0.3, 0.1],
    };

    let rubber = Material {
        diffuse: vector![0.3, 0.1, 0.1],
    };

    let b = Material {
        diffuse: vector![0.1, 0.1, 0.3],
    };

    scene.add_object(
        Sphere {
            center: vector![-3.0, 0.0, -16.0],
            radius: 2.0,
        },
        ivory,
    );

    scene.add_object(
        Sphere {
            center: vector![-1.0, -1.5, -12.0],
            radius: 2.0,
        },
        rubber,
    );

    scene.add_object(
        Sphere {
            center: vector![1.5, -0.5, -18.0],
            radius: 3.0,
        },
        rubber,
    );

    scene.add_object(
        Sphere {
            center: vector![7.0, 5.0, -18.0],
            radius: 4.0,
        },
        b,
    );

    scene.add_light(Light {
        position: vector![-20.0, 20.0, 20.0],
        intensity: 1.5,
    });

    scene
}

fn main() {
    let mut window = Window::new("Raytracer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut fb = Framebuffer::new(WIDTH, HEIGHT);

    let mut raytracer = Raytracer::new(&mut fb);
    raytracer.draw_scene(&example_scene());

    let _ = window.update_with_buffer(&fb.buffer, fb.width(), fb.height());

    while window.is_open() {
        window.update();
    }
}
