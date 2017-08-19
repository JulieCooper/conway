use std::str::FromStr;

#[derive(Clone)]
pub enum InputCells {
    Custom(Vec<(i8, i8)>),
    Neighbors,
    NoCorners,
    CornersOnly,
    FarOnly,
    FarAlso,
    FarCornersOnly,
}
impl FromStr for InputCells {
    type Err = ();
    fn from_str(s: &str) -> Result<InputCells, ()> {
        match s {
            "Neighbors" => Ok(InputCells::Neighbors),
            "NoCorners" => Ok(InputCells::NoCorners),
            "CornersOnly" => Ok(InputCells::CornersOnly),
            "FarOnly" => Ok(InputCells::FarOnly),
            "FarAlso" => Ok(InputCells::FarAlso),
            "FarCornersOnly" => Ok(InputCells::FarCornersOnly),
            _ => Err(()),
        }
    }
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
            InputCells::NoCorners => {
                vec![
                         (0,-1),        
                (-1, 0),         (1, 0),
                         (0, 1)        
                ]
            },
            InputCells::CornersOnly => {
                vec![
                (-1,-1),         (1,-1),
                                        
                (-1, 1),         (1, 1)
                ]
            },
            InputCells::FarOnly => {
                vec![
                (-2,-2), (-1,-2), (0,-2), (1,-2), (2,-2),
                (-2,-1),                          (2,-1),
                (-2, 0),                          (2, 0),
                (-2, 1),                          (2, 1),
                (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2)
                ]
            },
            InputCells::FarAlso => {
                vec![
                (-2,-2), (-1,-2), (0,-2), (1,-2), (2,-2),
                (-2,-1), (-1,-1), (0,-1), (1,-1), (2,-1),
                (-2, 0), (-1, 0),         (1, 0), (2, 0),
                (-2, 1), (-1, 1), (0, 1), (1, 1), (2, 1),
                (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2)
                ]
            },
            InputCells::FarCornersOnly => {
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
