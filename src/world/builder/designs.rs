pub enum Design {
    //Still lifes
    Block,
    Beehive,
    Loaf,
    Boat,
    Tub,
    //Oscillators
    Blinker,
    Toad,
    Beacon,
    //Spaceships
    Glider,
    LWSS,
}
/*
            Design:: => {
                vec![
                ( 1, 1), ( 2, 1), (3, 1), (4, 1), (5, 1),
                ( 1, 2), ( 2, 2), (3, 2), (4, 2), (5, 2),
                ( 1, 3), ( 2, 3), (3, 3), (4, 3), (5, 3),
                ( 1, 4), ( 2, 4), (3, 4), (4, 4), (5, 4),
                ( 1, 5), ( 2, 5), (3, 5), (4, 5), (5, 5)
                ]
            }
*/
impl Design {
    pub fn get_data(&self) -> Vec<(u64, u64)> {
        match *self {
            Design::Block => {
                vec![
                ( 1, 1), ( 2, 1),
                ( 1, 2), ( 2, 2)
                ]
            },
            Design::Beehive => {
                vec![
                         ( 2, 1), (3, 1),        
                ( 1, 2),                  (4, 2),
                         ( 2, 3), (3, 3)
                ]
            }
            Design::Loaf => {
                vec![
                         ( 2, 1), (3, 1),         
                ( 1, 2),                  (4, 2), 
                         ( 2, 3),         (4, 3), 
                                  (3, 4)
                ]
            }
            Design::Boat => {
                vec![
                ( 1, 1), ( 2, 1),        
                ( 1, 2),          (3, 2),
                         ( 2, 3) 
                ]
            }
            Design::Tub => {
                vec![
                         ( 2, 1),
                ( 1, 2),          (3, 2),
                ( 1, 3), ( 2, 3)
                ]
            }
            Design::Blinker => {
                vec![
                ( 1, 1), ( 2, 1), (3, 1)
                ]
            }
            Design::Toad => {
                vec![
                         ( 2, 1), (3, 1), (4, 1),
                ( 1, 2), ( 2, 2), (3, 2),
                ]
            }
            Design::Beacon => {
                vec![
                ( 1, 1), ( 2, 1),
                ( 1, 2),          
                                          (4, 3),
                                  (3, 4), (4, 4),
                ]
            }
            Design::LWSS => {
                vec![
                ( 1, 1),                  (4, 1),
                                                  (5, 2),
                ( 1, 3),                          (5, 3),
                         ( 2, 4), (3, 4), (4, 4), (5, 4),
                ]
            }
            Design::Glider => vec![(2, 1), (1, 3), (2, 3), (3, 3), (3, 2)],
                _ => vec![],
        }
    }
}
