use nalgebra::vector;
use raytracer::*;
use window::*;

mod raytracer;
mod window;

const WIDTH: usize = 1000;
const HEIGHT: usize = (WIDTH as f32 / (16.0 / 9.0)) as usize;

fn main() {
    let mut window = Window::new("Raytracer", WIDTH, HEIGHT);
    let mut raytracer = Raytracer::new(window.framebuffer_mut());

    // raytracer.draw_shape()

    raytracer.draw(|pos| {
        Color::from_f32(vector![
            pos[0] as f32 / WIDTH as f32,
            pos[1] as f32 / HEIGHT as f32,
            0.7
        ])
    });

    while window.is_open() {
        window.update_buffer()
    }
}
