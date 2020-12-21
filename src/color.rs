use crate::vec3::Vec3;

pub fn write_color(color: Vec3) {
    println!("{} {} {}", (color.x * 255.999) as u64, (color.y * 255.999) as u64, (color.z * 255.999) as u64)
}
