use std::fs;

pub fn create() {
    let mut contents = String::with_capacity(800_000);

    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    contents.push_str("P3\n");
    contents.push_str(&format!("{image_width} {image_height}\n"));
    contents.push_str("255\n");

    for j in 0..image_height {
        for i in 0..image_width {
            let r = f64::from(i) / f64::from(image_width - 1);
            let g = f64::from(j) / f64::from(image_height - 1);
            let b = 0.0;

            let ir     = (255.999 * r).floor() as i32;
            let ig  = (255.999 * g).floor() as i32;
            let ib  = (255.999 * b) as i32;

            contents.push_str(&format!("{ir} {ig} {ib}\n"));
        }
    }

    fs::create_dir_all("out").expect("Unable to create directory");
    fs::write("out/image.ppm", contents).expect("Unable to write file");

    print!("\rDone.                 \n");
}
