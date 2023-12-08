use std::{fs, io};
use std::io::Write;
use crate::vec3::Color;

pub fn create() {
    let mut contents = String::with_capacity(1_000_000);

    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    contents.push_str("P3\n");
    contents.push_str(&format!("{image_width} {image_height}\n"));
    contents.push_str("255\n");

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width as f64 - 1.0),
                j as f64 / (image_height as f64 - 1.0),
                0.0,
            );
            contents.push_str(&format!("{pixel_color}\n"));
        }
    }

    fs::create_dir_all("out").expect("Unable to create directory");
    fs::write("out/image.ppm", contents).expect("Unable to write file");

    print!("\rDone.                 \n");
}
