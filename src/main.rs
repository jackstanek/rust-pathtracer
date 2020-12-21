mod color;
mod vec3;
mod ray;

use crate::vec3::Vec3;
use crate::color::write_color;

fn main() -> std::io::Result<()> {
    let (width, height) = (256, 256);
    let depth: f64 = 255.999;

    /* Write the magic PPM header; "P3", then the width, height, and color depth. */
    println!("P3\n{} {}\n{}\n", width, height, depth.floor() as u32);

    for j in 0..height {
        for i in 0..width {
            let pixel_color = Vec3::new((i as f64) / ((width - 1) as f64), 
                                        (j as f64) / ((height - 1) as f64),
                                        0.25);
            write_color(pixel_color)
        }
    }

    eprintln!("Done");
    Ok(())
}
