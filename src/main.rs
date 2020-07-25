use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::Path;

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
            let r = f64::from(i) / f64::from(IMAGE_WIDTH - 1);
            let g = f64::from(j) / f64::from(IMAGE_HEIGHT - 1);
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            writeln!(&mut file, "{} {} {}", ir, ig, ib)?;
        }
    }

    file.flush()?;

    println!("Done.");
    Ok(())
}
