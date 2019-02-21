extern crate image;

mod cli;
mod action;
use action::Action::*;
use action::Flag;
use image::*;
use std::collections::HashMap;
use std::sync::mpsc;

fn main() {
    let (io, settings, image_names) = cli::parse();

    // TODO: If you append the same image as source it'll error, i should check if already exists
    // or something
    let mut images: HashMap<String, mpsc::Receiver<DynamicImage>> = HashMap::new();
    for image_name in image_names {

        let (s, r) = mpsc::channel();
        let i_n = image_name.clone();
        std::thread::spawn(move|| {
            s.send(image::open(&i_n)
                .map_err(|e| {
                    println!("{}", e);
                })
                .expect("Aborting because one or more errors while loading image")).unwrap();
        });
        images.insert(image_name, r);
    };

    let mut image = images.get_mut(&io.0).unwrap().recv().unwrap();
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

                /* This should be made at cli.rs and images.get(filename).recv().unwrap() here instead
                let mut image_to_append = image::open(&filename)
                    .map_err(|e| println!("{}: {}", &filename, e))
                    .expect("Aborting due to errors while opening image");
                */

                let mut image_to_append = images.get_mut(&filename).unwrap().recv().unwrap();

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
