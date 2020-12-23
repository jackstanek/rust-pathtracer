mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use crate::vec3::{Vec3,Color};
use crate::color::write_color;
use crate::hittable::{Hittable,HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    match world.hit(ray, 0.0, 100.0) {
        Some(hr) => {
            Color::new(hr.normal.x() + 1.0, hr.normal.y() + 1.0, hr.normal.z() + 1.0) * 0.5
        },
        None => {
            let t = 0.5 * (ray.direction().y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
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
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(0.0, 0.0, -1.0, 0.5)));
    world.add(Box::new(Sphere::new(0.0, -100.5, -1.0, 100.0)));

    /* Write the magic PPM header; "P3", then the width, height, and color depth. */
    println!("P3\n{} {}\n{}\n", width, height, depth.floor() as u64);

    for j in 0..height {
        for i in 0..width {
            let u = i as f64 / (width - 1) as f64;
            let v = (height - j) as f64 / (height - 1) as f64;
            let r = Ray::new(&origin, &(lower_left_corner + (horizontal * u) + (vertical * v) - origin));
            let pixel_color = ray_color(&r, &world);
            write_color(pixel_color)
        }
    }

    eprintln!("Done");
    Ok(())
}
