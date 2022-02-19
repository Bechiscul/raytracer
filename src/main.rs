use nalgebra::vector;
use raytracer::*;
use scene::*;
use window::*;

mod raytracer;
mod scene;
mod window;

const WIDTH: usize = 1000;
const HEIGHT: usize = (WIDTH as f32 / (16.0 / 9.0)) as usize;

// Optimization techniques:
// - Multithreading
// - Sort-by-depth algorithm
// - Tagged-unions.

fn main() {
    let mut window = Window::new("Raytracer", WIDTH, HEIGHT);

    let mut scene = Scene::new();

    scene.add_object(Sphere {
        center: vector![-3.0, 0.0, -16.0],
        radius: 2.0,
    });

    scene.add_object(Sphere {
        center: vector![-1.0, -1.5, -12.0],
        radius: 2.0,
    });

    scene.add_object(Sphere {
        center: vector![1.5, -0.5, -18.0],
        radius: 3.0,
    });

    scene.add_object(Sphere {
        center: vector![7.0, 5.0, -18.0],
        radius: 4.0,
    });

    let mut raytracer = Raytracer::new(window.framebuffer_mut());
    raytracer.draw_scene(&scene);

    while window.is_open() {
        window.update_buffer()
    }
}

/*

let shapes = sortByDepth(shapes)

for shape in shapes:
    raytracer.draw(|pos| {
        let ray = Ray();
    })
    if shape.intersects():


*/
