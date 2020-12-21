mod color;
mod vec3;
mod ray;

use crate::vec3::{Vec3,Color};
use crate::color::write_color;
use crate::ray::Ray;

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
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
