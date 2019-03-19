extern crate image;

mod action;
mod cli;
use action::Action::*;
use action::{Direction, Flag, Orientation};
use image::*;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::process::exit;
use std::sync::mpsc;

fn main() {
    let (io, settings, image_names) = cli::parse();

    // Load images
    let mut images: HashMap<String, mpsc::Receiver<DynamicImage>> = HashMap::new();
    let mut extra_images: HashMap<String, DynamicImage> = HashMap::new();
    for image_name in image_names {
        if images.contains_key(&image_name) {
            continue;
        }
        let (s, r) = mpsc::channel();
        let i_n = image_name.clone();
        std::thread::spawn(move || {
            s.send(
                image::open(&i_n)
                    .map_err(|e| {
                        eprintln!("{}", e);
                    })
                    .unwrap_or_else(|_| {
                        eprintln!("Aborting because one or more errors while loading image");
                        exit(2)
                    }),
            )
            .unwrap();
        });
        images.insert(image_name, r);
    }

    // Use extension of outfile as default, can be overwritten with format: action
    let outname = io.1.clone().to_owned();
    let gutted_outname: Vec<&str> = outname.split(".").collect();
    let mut out_format = match gutted_outname[gutted_outname.len() - 1] {
        "png" => ImageOutputFormat::PNG,
        "jpg" => ImageOutputFormat::JPEG(100),
        "jpeg" => ImageOutputFormat::JPEG(100),
        "bmp" => ImageOutputFormat::BMP,
        "gif" => ImageOutputFormat::GIF,
        "ico" => ImageOutputFormat::ICO,
        &_ => ImageOutputFormat::PNG,
    };

    let mut image = images.get_mut(&io.0).unwrap().recv().unwrap();
    for action in settings.actions {
        match action {
            Invert => image.invert(),

            Contrast(c) => image = image.adjust_contrast(c),

            Brightness(b) => image = image.brighten(b),

            Blur(b) => image = image.blur(b),

            Unsharpen(sigma, threshold) => image = image.unsharpen(sigma, threshold),

            Crop(x, y, w, h) => image = image.crop(x, y, w, h),

            Rotate(d) => {
                image = match d {
                    Direction::Right => image.rotate90(),
                    Direction::Left => image.rotate270(),
                    Direction::Down => image.rotate180(),
                    Direction::Up => image,
                }
            }

            Flip(orientation) => match orientation {
                Orientation::Vertical => image = image.flipv(),
                Orientation::Horizontal => image = image.fliph(),
            },

            Scale(w, h) => {
                // Grab which algorithm to use from flag
                let algorithm = if cli::flag_is_enabled(settings.flags.get(&Flag::Lanczos3)) {
                    Lanczos3
                } else {
                    Nearest
                };
                if w == 0 {
                    image = image.resize(std::u32::MAX, h, algorithm);
                    continue;
                }
                if h == 0 {
                    image = image.resize(w, std::u32::MAX, algorithm);
                    continue;
                }
                image = image.resize_exact(w, h, algorithm)
            }

            Append(filename, direction) => {
                // Grab which algorithm to use from flag
                let algorithm = if cli::flag_is_enabled(settings.flags.get(&Flag::Lanczos3)) {
                    Lanczos3
                } else {
                    Nearest
                };

                // The appendable image can either be same as source, an image that hasn't been
                // initialized, or an already initialized one. This handles all 3 cases
                let mut image_to_append;
                if filename == io.0 {
                    image_to_append = image.clone();
                } else {
                    if !extra_images.contains_key(&filename) {
                        extra_images.insert(
                            filename.clone(),
                            images.get_mut(&filename).unwrap().recv().unwrap(),
                        );
                    }
                    image_to_append = extra_images.get_mut(&filename).unwrap().clone();
                }

                // Appended image inherits size of original image
                let mut parent = if direction == Direction::Up || direction == Direction::Down {
                    // Vertically append
                    image_to_append = image_to_append.resize(image.width(), 100000000, algorithm);
                    image::DynamicImage::new_rgba8(
                        image.width(),
                        image.height() + image_to_append.height(),
                    )
                } else {
                    // Horizontally append
                    image_to_append = image_to_append.resize(100000000, image.height(), algorithm);
                    image::DynamicImage::new_rgba8(
                        image.width() + image_to_append.width(),
                        image.height(),
                    )
                };

                match direction {
                    Direction::Up => {
                        parent.copy_from(&image_to_append, 0, 0);
                        parent.copy_from(&image, 0, image_to_append.height());
                    }
                    Direction::Down => {
                        parent.copy_from(&image, 0, 0);
                        parent.copy_from(&image_to_append, 0, image.height());
                    }
                    Direction::Left => {
                        parent.copy_from(&image_to_append, 0, 0);
                        parent.copy_from(&image, image_to_append.width(), 0);
                    }
                    Direction::Right => {
                        parent.copy_from(&image, 0, 0);
                        parent.copy_from(&image_to_append, image.width(), 0);
                    }
                }
                image = parent;
            }
            Format(f) => out_format = f,
        };
    }

    match io.1.as_ref() {
        "stdout" => image
            .write_to(&mut std::io::stdout(), out_format)
            .unwrap_or_else(|e| {
                eprintln!("Failed to save image: {}", e);
                exit(2)
            }),
        _ => image
            .write_to(
                &mut File::create(&io.1).unwrap_or_else(|_| {
                    eprintln!("Outfile {} not found", io.1);
                    exit(2)
                }),
                out_format,
            )
            .unwrap_or_else(|e| {
                eprintln!("Failed to save image: {}", e);
                exit(2)
            }),
    }
}
