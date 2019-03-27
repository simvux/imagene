extern crate image;
extern crate imageproc;
extern crate rusttype;

use image::{DynamicImage, GenericImageView, ImageRgba8};
use rusttype::FontCollection;

use std::fs;

pub fn draw(
    mut image: DynamicImage, // Taking ownership is fine since it returns it back
    rgba: (u8, u8, u8, u8),
    font: &str,
    (x, y): (f32, f32),
    scale: f32,
    text: &str,
) -> DynamicImage {
    let color = image::Rgba {
        data: [rgba.0, rgba.1, rgba.2, rgba.3],
    };
    let font = match load_font(&font) {
        Err(e) => {
            eprintln!("{:?}", e);
            return image;
        }
        Ok(f) => f,
    };
    let (w, h) = (image.width() as f32, image.height() as f32);
    ImageRgba8(imageproc::drawing::draw_text(
        &mut image,
        color,
        (w * x) as u32,
        (h * y) as u32,
        rusttype::Scale::uniform(w as f32 * (scale * 0.1)),
        &font,
        text,
    ))
}

fn load_font(name: &str) -> Result<rusttype::Font, ()> {
    let bytes = fs::read(name).map_err(|e| {
        eprintln!("loading {}: {}", name, e);
    });
    if bytes.is_err() {
        return Err(());
    }

    Ok(FontCollection::from_bytes(bytes.unwrap())
        .map_err(|e| {
            eprintln!("{}", e);
        })
        .unwrap()
        .into_font()
        .map_err(|e| {
            eprintln!("{}", e);
        })
        .unwrap())
}
