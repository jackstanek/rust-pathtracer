use crate::vec3::Vec3;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    max.min(x.max(min))
}

pub fn write_color(color: Vec3, samples_per_pixel: u64) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (color.x() * scale).sqrt();
    let g = (color.y() * scale).sqrt();
    let b = (color.z() * scale).sqrt();

    println!("{} {} {}", 
             (clamp(r, 0., 0.999) * 256.) as u64,
             (clamp(g, 0., 0.999) * 256.) as u64,
             (clamp(b, 0., 0.999) * 256.) as u64)
}
