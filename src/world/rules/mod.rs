pub mod input_cells;
use self::input_cells::Input_Cells;
use world::CellState;
use std::collections::HashMap;

type Ruleset_Type = HashMap<(CellState, CellState), Vec<(u8, CellState)>>;
pub enum Rulesets {
    Custom,
    Conway,
}
impl Rulesets {
    fn get_data(&self) -> DSL_Ruleset {
        use self::CellState::{Dead, Live};
        match self {
            //Custom will use yet-formalized DSL that will be used for config files
            Custom => {
                let mut ruleset = DSL_Ruleset::new();
                ruleset.add_cases(Dead, Live, vec![(3, Live)]);
                ruleset
            },
            Conway => {
                let mut ruleset = DSL_Ruleset::new();
                ruleset.add_cases(Dead, Live, vec![(3, Live)]);
                ruleset.add_cases(Live, Live, vec![
                    (0, Dead), (1, Dead), (4, Dead), (5, Dead),
                    (6, Dead), (7, Dead), (8, Dead), (9, Dead),
                ]);
                ruleset
            },
        }
    }
}
pub struct DSL_Ruleset {
    data: Ruleset_Type,
}
impl DSL_Ruleset {
    pub fn new() -> Self {
        DSL_Ruleset {
            data: HashMap::new(),
        }
    }
    pub fn add_cases(&mut self,
                 identity_state: CellState,
                 for_state: CellState,
                 cases: Vec<(u8, CellState)>) {
        let key = (identity_state, for_state);

        self.data.insert(key, cases);
    }
    //figure out how to use references here.......
    fn compute(&mut self,
               identity_state: CellState,
               input_states: Vec<(CellState, u8)>) -> CellState {
        use self::CellState::{Dead, Live};
        //let mut ruleset = DSL_Ruleset::new();

        //ruleset.add_cases(Dead, Live, vec![(3, Live)]);
        self.add_cases(Dead, Live, vec![(3, Live)]);

        let mut return_state = identity_state.clone();
        for input_state in input_states.iter() {
            let key = (identity_state.clone(), input_state.0.clone());
            match self.data.get(&key) {
                Some(vec) => {
                    for rule_state in vec {
                        if rule_state.0 == input_state.1 {
                            return_state = rule_state.1.clone();
                        }
                    }
                },
                None => (),
            }
        }

        return_state
    }
}
pub enum Cell_Rules {
    Conway,
}
impl Cell_Rules {
    fn get_data(&self) -> () {
        match *self {
            Cell_Rules::Conway => (),
        }
    }
}
pub struct Ruleset {
    pub input_cells: Input_Cells,
    pub rules: Box<FnMut(&CellState, &Vec<CellState>) -> CellState>,
}
