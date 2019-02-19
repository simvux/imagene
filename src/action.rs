pub enum Action {
    Contrast(f32),
    Scale(u32, u32),
    Append(String),
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Flag {
    Shrink,
}
