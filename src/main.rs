mod color;
mod vec3;
mod ray;

use std::option::Option;

use crate::vec3::{Vec3,Color};
use crate::color::write_color;
use crate::ray::Ray;

fn intersects_sphere(center: &Vec3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin() - *center;
    let dir = ray.direction();
    let a = dir.dot(&dir);
    let b = oc.dot(&dir) * 2.0;
    let c = oc.dot(&oc) - radius.powi(2);
    let discrim = b.powi(2) - 4.0 * a * c;
    if discrim > 0.0 {
        Some((-b - discrim.sqrt()) / (2.0 * a))
    } else {
        None
    }
}

fn ray_color(ray: &Ray) -> Color {
    match intersects_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        Some(t) => {
            let N = (ray.point_at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
            Color::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) * 0.5
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
    let width: u64 = 720;
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

    /* Write the magic PPM header; "P3", then the width, height, and color depth. */
    println!("P3\n{} {}\n{}\n", width, height, depth.floor() as u64);

    for j in 0..height {
        for i in 0..width {
            let u = i as f64 / (width - 1) as f64;
            let v = (height - j) as f64 / (height - 1) as f64;
            let r = Ray::new(&origin, &(lower_left_corner + (horizontal * u) + (vertical * v) - origin));
            let pixel_color = ray_color(&r);
            write_color(pixel_color)
        }
    }

    eprintln!("Done");
    Ok(())
}
