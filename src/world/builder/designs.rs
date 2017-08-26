#[derive(Clone)]
pub enum Design {
    WireTest,
}
impl Design {
    pub fn get_data(&self) -> Vec<(u64, u64, usize)> {
        match *self {
            Design::WireTest => { vec![
                (2, 1, 2), (3, 1, 3), (4, 1, 3), (5, 1, 3), (6, 1, 3), (7, 1, 3), (8, 1, 3),
                (9, 1, 3), (10, 1, 3), (11, 1, 3), (12, 1, 3), (13, 1, 3), (14, 1, 3),
                (1, 2, 3), (2, 2, 1),
            ]},
        }
    }
}
