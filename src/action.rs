use crate::cli;
use image::{DynamicImage, FilterType::*, GenericImage, GenericImageView, ImageOutputFormat};
use std::{collections::HashMap, sync::mpsc};

pub enum Action {
    Invert,
    Blur(f32),
    Brightness(i32),
    Contrast(f32),
    Rotate(Direction),
    Crop(u32, u32, u32, u32),
    Unsharpen(f32, i32),
    Scale(u32, u32),
    Append(String, Direction),
    Flip(Orientation),
    Format(image::ImageOutputFormat),
}

pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Flag {
    Lanczos3,
}

pub fn apply_actions(
    infile: &str,
    mut out_format: ImageOutputFormat,
    actions: Vec<Action>,
    flags: HashMap<Flag, bool>,
    mut images: HashMap<String, mpsc::Receiver<DynamicImage>>,
) -> (DynamicImage, ImageOutputFormat) {
    let mut image = images.get_mut(infile).unwrap().recv().unwrap();
    let mut extra_images: HashMap<String, DynamicImage> = HashMap::new();

    for action in actions {
        match action {
            Action::Invert => image.invert(),

            Action::Contrast(c) => image = image.adjust_contrast(c),

            Action::Brightness(b) => image = image.brighten(b),

            Action::Blur(b) => image = image.blur(b),

            Action::Unsharpen(sigma, threshold) => image = image.unsharpen(sigma, threshold),

            Action::Crop(x, y, w, h) => image = image.crop(x, y, w, h),

            Action::Rotate(d) => {
                image = match d {
                    Direction::Right => image.rotate90(),
                    Direction::Left => image.rotate270(),
                    Direction::Down => image.rotate180(),
                    Direction::Up => image,
                }
            }

            Action::Flip(orientation) => match orientation {
                Orientation::Vertical => image = image.flipv(),
                Orientation::Horizontal => image = image.fliph(),
            },

            Action::Scale(w, h) => {
                // Grab which algorithm to use from flag
                let algorithm = if cli::flag_is_enabled(flags.get(&Flag::Lanczos3)) {
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

            Action::Append(filename, direction) => {
                // Grab which algorithm to use from flag
                let algorithm = if cli::flag_is_enabled(flags.get(&Flag::Lanczos3)) {
                    Lanczos3
                } else {
                    Nearest
                };

                // The appendable image can either be same as source, an image that hasn't been
                // initialized, or an already initialized one. This handles all 3 cases
                let mut image_to_append;
                if filename == infile {
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
                    image_to_append =
                        image_to_append.resize(image.width(), std::u32::MAX, algorithm);
                    image::DynamicImage::new_rgba8(
                        image.width(),
                        image.height() + image_to_append.height(),
                    )
                } else {
                    // Horizontally append
                    image_to_append =
                        image_to_append.resize(std::u32::MAX, image.height(), algorithm);
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
            Action::Format(f) => out_format = f,
        };
    }
    (image, out_format)
}
