mod vec3;
mod color;

use crate::color::{write_color, Color};

fn main() {

    // Image

    let image_width: i32 = 256;
    let image_height: i32 = 256;

    //Render

    let mut stdout = std::io::stdout();

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height-j);
        for i in 0..image_width {
            let pixel_color = Color::new((i as f64)/((image_width-1) as f64), (j as f64)/((image_height-1) as f64), 0.0);
            write_color(&mut stdout, &pixel_color);
        }
    }
    eprintln!("\rDone.                 ");
}