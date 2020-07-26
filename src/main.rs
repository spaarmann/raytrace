use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::Path;

mod vec3;
use vec3::Vec3;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(&mut file, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanline {}/{}", IMAGE_HEIGHT - j, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let color = Vec3(
                f64::from(i) / f64::from(IMAGE_WIDTH - 1),
                f64::from(j) / f64::from(IMAGE_HEIGHT - 1),
                0.25,
            );

            write_color(&mut file, color)?;
        }
    }

    file.flush()?;

    println!("Done.");
    Ok(())
}

fn write_color<F: Write>(f: &mut F, color: Vec3) -> std::io::Result<()> {
    writeln!(
        f,
        "{} {} {}",
        (255.999 * color.0) as u32,
        (255.999 * color.1) as u32,
        (255.999 * color.2) as u32
    )
}
