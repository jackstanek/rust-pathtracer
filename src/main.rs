mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

extern crate rand;
use rand::{SeedableRng, Rng};
use rand::rngs::SmallRng;

extern crate minifb;
use minifb::{Key, Window, WindowOptions};

use crate::camera::Camera;
use crate::vec3::{Color,Point,Vec3};
use crate::hittable::{Hittable,HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    max.min(x.max(min))
}

fn color_to_fb_pixel(color: &Color) -> u32 {
    const DEPTH: f64 = 256.;
    let r = (clamp((color.x()).sqrt(), 0., 0.999) * DEPTH) as u32;
    let g = (clamp((color.y()).sqrt(), 0., 0.999) * DEPTH) as u32;
    let b = (clamp((color.z()).sqrt(), 0., 0.999) * DEPTH) as u32;

    (r << 16) | (g << 8) | b
}

fn background_color(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction().y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u8) -> Color {
    if depth > 50 {
        return Color::new(0.0, 0.0, 0.0)
    }

    match world.hit(ray, 0.001, f64::INFINITY) {
        Some(hr) => {
            let target = hr.point + hr.normal + Point::new_rand_in_sphere();
            let new_ray = Ray::new(&hr.point, &(target - hr.point));
            ray_color(&new_ray, world, depth + 1) * 0.5
        },
        None => {
            background_color(ray)
        }
    }
}

fn main() -> std::io::Result<()> {
    /* Image setup */
    let aspect_ratio = 16.0 / 9.0;
    let width: usize = 640;
    let height: usize = (width as f64 / aspect_ratio) as usize;

    /* Camera setup. TODO: Factor this out somehow. */
    let viewport_height = 2.0;
    let focal_length = 1.0;

    let camera = Camera::new(viewport_height, aspect_ratio * viewport_height, focal_length);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(0.0, 0.0, -1.0, 0.5)));
    world.add(Box::new(Sphere::new(0.0, -100.5, -1.0, 100.0)));

    let mut framebuffer: Vec<u32> = vec![0; width * height];
    let mut color_samples: Vec<Color> = Vec::with_capacity(width * height);

    let mut rng = SmallRng::from_rng(rand::thread_rng()).unwrap();
    let samples_per_pixel = 1000;

    let mut window = Window::new(
        "Rust Pathtracer",
        width,
        height,
        WindowOptions::default()
    ).unwrap_or_else(|e| {
        panic!("{}", e)
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut finished_rendering = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if finished_rendering {
            window.update();
            continue;
        }

        for s in 0..samples_per_pixel {
            for j in 0..height {
                for i in 0..width {
                    let u = (i as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                    let v = ((height - j) as f64 + rng.gen::<f64>()) / (height - 1) as f64;
                    let r = camera.get_ray(u, v);
                    let color = ray_color(&r, &world, 0);

                    let index = width * j + i;
                    if index < color_samples.len() {
                        color_samples[index] = (color_samples[index] * s as f64 + color) / (s as f64 + 1.);
                    } else {
                        color_samples.push(color);
                    }
                }
            }

            for i in 0..(&color_samples).len() {
                framebuffer[i] = color_to_fb_pixel(&color_samples[i]);
            }

            window.update_with_buffer(&framebuffer, width, height).unwrap();
            eprint!("\rRendered {} samples", s + 1);
        }

        finished_rendering = true;
        eprintln!("\nFinished.")
    }
    Ok(())
}
