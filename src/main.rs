use nalgebra::vector;
use raytracer::*;
use scene::*;
use window::*;

mod raytracer;
mod scene;
mod window;

const WIDTH: usize = 1000;
const HEIGHT: usize = (WIDTH as f32 / (16.0 / 9.0)) as usize;

fn main() {
    let mut window = Window::new("Raytracer", WIDTH, HEIGHT);

    let mut scene = Scene::default();
    // scene.add_object()

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
