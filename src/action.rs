pub enum Action {
    Blur(f32),
    Brightness(i32),
    Contrast(f32),
    Rotate(Direction),
    Crop(u32, u32, u32, u32),
    Unsharpen(f32, i32),
    Scale(u32, u32),
    Append(String),
    Flip(Orientation),
}

pub enum Orientation {
    Vertical,
    Horizontal,
}
pub enum Direction {
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Flag {
    Shrink,
    Vertical,
}
