use framebuffer::*;
use minifb::*;
use nalgebra::{vector, Vector3, Vector4};
use raytracer::*;
use scene::*;

mod framebuffer;
mod raytracer;
mod scene;

const WIDTH: usize = 1000;
const HEIGHT: usize = (WIDTH as f32 / (16.0 / 9.0)) as usize;

fn example_scene() -> Scene {
    let mut scene = Scene::new();

    let chrome = Material {
        ambient: vector![0.25, 0.25, 0.25],
        diffuse: vector![0.4, 0.4, 0.4],
        specular: vector![0.774597, 0.774597, 0.774597],
        shininess: 0.6,
    };

    let bronze = Material {
        ambient: vector![0.2125, 0.1275, 0.054],
        diffuse: vector![0.714, 0.4284, 0.18144],
        specular: vector![0.393548, 0.271906, 0.166721],
        shininess: 0.2,
    };

    let red_plasic = Material {
        ambient: vector![0.0, 0.0, 0.0],
        diffuse: vector![0.5, 0.0, 0.0],
        specular: vector![0.7, 0.6, 0.6],
        shininess: 0.25,
    };

    let mirror = Material {
        ambient: vector![0.0, 0.0, 0.0],
        diffuse: vector![0.5, 0.0, 0.0],
        specular: vector![0.2, 0.2, 0.2],
        shininess: 1425.0,
    };

    scene.add_object(
        Sphere {
            center: vector![-3.0, 0.0, -16.0],
            radius: 2.0,
        },
        chrome,
    );

    scene.add_object(
        Sphere {
            center: vector![-1.0, -1.5, -12.0],
            radius: 2.0,
        },
        mirror,
    );

    scene.add_object(
        Sphere {
            center: vector![1.5, -0.5, -18.0],
            radius: 3.0,
        },
        bronze,
    );

    scene.add_object(
        Sphere {
            center: vector![7.0, 5.0, -18.0],
            radius: 4.0,
        },
        mirror,
    );

    scene.add_light(Light {
        position: vector![-20.0, 20.0, 20.0],
        ambient: Vector3::<f32>::repeat(1.0),
        diffuse: Vector3::<f32>::repeat(1.0),
        specular: Vector3::<f32>::repeat(1.0),
    });

    scene.background = Some(vector![0.2, 0.7, 0.8]);
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
