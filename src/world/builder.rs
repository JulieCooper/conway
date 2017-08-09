pub enum Design {
    Horizontal_Lines,
    Vertical_Lines,
}
pub enum InitialState {
    Random,
    Blank,
    Ordered(Vec<(u64, u64)>),
    Preconstructed(Design),
    Lambda,
}
