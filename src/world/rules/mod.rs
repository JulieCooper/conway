pub mod input_cells;
use self::input_cells::Input_Cells;
use world::CellState;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

type Ruleset_Type = HashMap<(CellState, CellState), Vec<(usize, CellState)>>;
pub enum Rulesets {
    Custom,
    Conway,
}
pub struct Ruleset {
    pub input_cells: Input_Cells,
    pub rules: Rulesets,//Box<FnMut(&CellState, &Vec<CellState>) -> CellState>,
}
impl Rulesets {
    pub fn get_data(&self) -> DSL_Ruleset {
        use self::CellState::{Dead, Live};
        match *self {
            //Custom will use yet-formalized DSL that will be used for config files
            Rulesets::Custom => {
                let mut ruleset = DSL_Ruleset::new();
                ruleset.add_cases(Dead, Live, vec![(3, Live)]);
                ruleset
            },
            Rulesets::Conway => {
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
                 cases: Vec<(usize, CellState)>) {
        let key = (identity_state, for_state);

        self.data.insert(key, cases);
    }
    //figure out how to use references here.......
    pub fn compute(&mut self,
               identity_state: CellState,
               input_states: Vec<(CellState, usize)>) -> CellState {
        let mut return_state = identity_state.clone();
        for state_amount in input_states.iter() {
            //key = (my_state, for_state)
            let key = (identity_state.clone(), state_amount.0.clone());
            match self.data.get(&key) {
                //Vec<(usize, CellState)>
                Some(vec) => {
                    for rule in vec {
                        if rule.0 == state_amount.1 {
                            return_state = rule.1.clone();
                        }
                    }
                },
                None => (),
            }
        }

        return_state
    }
}
