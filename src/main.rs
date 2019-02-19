extern crate image;

mod cli;
mod action;
use action::Action::*;
use action::Flag;
use image::*;

fn main() {
    let (io, settings) = cli::parse();

    let mut image = image::open(io.0)
        .map_err(|e| {
            println!("{}", e);
        })
        .expect("Aborting because one or more errors while loading image");

    for action in settings.actions {
        match action {
            Contrast(c) => image = image.adjust_contrast(c),
            Scale(w, h) => {
                if w == 0 {
                    image = image.resize(100000000, h, Nearest);
                    continue;
                }
                if h == 0 {
                    image = image.resize(w, 100000000, Nearest);
                    continue;
                }
                image = image.resize_exact(w, h, Nearest)
            }
            Append(filename) => {
                let mut image_to_append = image::open(&filename)
                    .map_err(|e| println!("{}: {}", &filename, e))
                    .expect("Aborting due to errors while opening image");

                /* Multithreadding || Just wait for the new async/await system
                 * If i take the percentual difference of height and width
                 * i can use that offset when looking at the numbural difference of both images in
                 * height, and multiply that by the percentual offset to precalculate the total
                 * width, using that data i can generate the new image with ::new_rgba8 on another
                 * thread while current ones are being rescaled
                 */
                let shrink_flag = match settings.flags.get(&Flag::Shrink) {
                    Some(f) => *f,
                    None => false,
                };
                let mut a_to_b;
                if image_to_append.height() > image.height() {
                    a_to_b = true;
                } else {
                    a_to_b = false;
                }
                if shrink_flag {
                    a_to_b = !a_to_b
                }
                if a_to_b {
                    image = image.resize(100000000, image_to_append.height(), Nearest);
                } else {
                    image_to_append = image_to_append.resize(100000000, image.height(), Nearest);
                };

                let mut parent = image::DynamicImage::new_rgba8(
                    image.width() + image_to_append.width(),
                    image.height(),
                );
                parent.copy_from(&image, 0, 0);
                parent.copy_from(&image_to_append, image.width(), 0);
                image = parent;
            }
        };
    }

    image.save(io.1).unwrap();
}
