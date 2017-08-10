pub enum Input_Cells {
    Custom(Vec<(i8, i8)>),
    Neighbors,
    No_Corners,
    Corners_Only,
    Far_Only,
    Far_Also,
    Far_Corners_Only,
}
impl Input_Cells {
    pub fn get_data(&self) -> Vec<(i8, i8)> {
        match *self {
            Input_Cells::Neighbors => {
                vec![
                (-1,-1), (0,-1), (1,-1),
                (-1, 0),         (1, 0),
                (-1, 1), (0, 1), (1, 1)
                ]
            },
            Input_Cells::No_Corners => {
                vec![
                         (0,-1),        
                (-1, 0),         (1, 0),
                         (0, 1)        
                ]
            },
            Input_Cells::Corners_Only => {
                vec![
                (-1,-1),         (1,-1),
                                        
                (-1, 1),         (1, 1)
                ]
            },
            Input_Cells::Far_Only => {
                vec![
                (-2,-2), (-1,-2), (0,-2), (1,-2), (2,-2),
                (-2,-1),                          (2,-1),
                (-2, 0),                          (2, 0),
                (-2, 1),                          (2, 1),
                (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2)
                ]
            },
            Input_Cells::Far_Also => {
                vec![
                (-2,-2), (-1,-2), (0,-2), (1,-2), (2,-2),
                (-2,-1), (-1,-1), (0,-1), (1,-1), (2,-1),
                (-2, 0), (-1, 0),         (1, 0), (2, 0),
                (-2, 1), (-1, 1), (0, 1), (1, 1), (2, 1),
                (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2)
                ]
            },
            Input_Cells::Far_Corners_Only => {
                vec![
                (-2,-2), (-1,-2),         (1,-2), (2,-2),
                (-2,-1),                          (2,-1),
                                                         
                (-2, 1),                          (2, 1),
                (-2, 2), (-1, 2),         (1, 2), (2, 2)
                ]
            }
            Input_Cells::Custom(ref coords) => coords.clone(),
        }
    }
}
