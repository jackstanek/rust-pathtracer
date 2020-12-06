use std::io::Write;
use std::fs::File;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of arguments.");
    } else {
        let mut file = File::create(args[1].as_str())?;
        let (width, height) = (256, 256);
        let depth: f64 = 255.999;

        /* Write the magic PPM header; "P3", then the width, height, and color depth. */
        write!(file, "P3\n{} {}\n{}\n", width, height, depth.floor() as u32)?;

        for j in 0..height {
            for i in 0..width {
                let r: f64 = (i as f64) / ((width - 1) as f64);
                let g: f64 = (j as f64) / ((height - 1) as f64);
                let b = 0.25;

                write!(file, "{} {} {}\n",
                       (r * depth) as u8,
                       (g * depth) as u8,
                       (b * depth) as u8)?;
            }
        }
    }

    Ok(())
}
