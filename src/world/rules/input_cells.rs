pub enum InputCells {
    Custom(Vec<(i8, i8)>),
    Neighbors,
    No_Corners,
    Corners_Only,
    Far_Only,
    Far_Also,
    Far_Corners_Only,
}
impl InputCells {
    pub fn get_data(&self) -> Vec<(i8, i8)> {
        match *self {
            InputCells::Neighbors => {
                vec![
                (-1,-1), (0,-1), (1,-1),
                (-1, 0),         (1, 0),
                (-1, 1), (0, 1), (1, 1)
                ]
            },
            InputCells::No_Corners => {
                vec![
                         (0,-1),        
                (-1, 0),         (1, 0),
                         (0, 1)        
                ]
            },
            InputCells::Corners_Only => {
                vec![
                (-1,-1),         (1,-1),
                                        
                (-1, 1),         (1, 1)
                ]
            },
            InputCells::Far_Only => {
                vec![
                (-2,-2), (-1,-2), (0,-2), (1,-2), (2,-2),
                (-2,-1),                          (2,-1),
                (-2, 0),                          (2, 0),
                (-2, 1),                          (2, 1),
                (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2)
                ]
            },
            InputCells::Far_Also => {
                vec![
                (-2,-2), (-1,-2), (0,-2), (1,-2), (2,-2),
                (-2,-1), (-1,-1), (0,-1), (1,-1), (2,-1),
                (-2, 0), (-1, 0),         (1, 0), (2, 0),
                (-2, 1), (-1, 1), (0, 1), (1, 1), (2, 1),
                (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2)
                ]
            },
            InputCells::Far_Corners_Only => {
                vec![
                (-2,-2), (-1,-2),         (1,-2), (2,-2),
                (-2,-1),                          (2,-1),
                                                         
                (-2, 1),                          (2, 1),
                (-2, 2), (-1, 2),         (1, 2), (2, 2)
                ]
            }
            InputCells::Custom(ref coords) => coords.clone(),
        }
    }
}
