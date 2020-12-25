mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

extern crate rand;
use rand::{SeedableRng, Rng};
use rand::rngs::SmallRng;

use crate::camera::Camera;
use crate::vec3::{Color,Point};
use crate::color::write_color;
use crate::hittable::{Hittable,HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;

fn background_color(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction().y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u8) -> Color {
    if depth > 50 {
        return Color::new(0.0, 0.0, 0.0)
    }

    match world.hit(ray, 0.0, f64::INFINITY) {
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
    let width: u64 = 640;
    let height: u64 = (width as f64 / aspect_ratio) as u64;
    let depth: f64 = 255.999;

    /* Camera setup. TODO: Factor this out somehow. */
    let viewport_height = 2.0;
    let focal_length = 1.0;

    let camera = Camera::new(viewport_height, aspect_ratio * viewport_height, focal_length);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(0.0, 0.0, -1.0, 0.5)));
    world.add(Box::new(Sphere::new(0.0, -100.5, -1.0, 100.0)));

    /* Write the magic PPM header; "P3", then the width, height, and color depth. */
    println!("P3\n{} {}\n{}\n", width, height, depth.floor() as u64);

    let mut rng = SmallRng::from_entropy();
    let samples_per_pixel = 2;
    for j in 0..height {
        for i in 0..width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                let v = ((height - j) as f64 + rng.gen::<f64>()) / (height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, 0);
            }
            write_color(pixel_color, samples_per_pixel)
        }
        eprint!("\rProgress: {:.1}%", (j as f64 / height as f64) * 100.0);
    }

    eprintln!("\nDone");
    Ok(())
}
