pub mod input_cells;
use self::input_cells::InputCells;
use world::CellState;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

type Ruleset_Type = HashMap<(CellState, CellState), Vec<(usize, CellState)>>;
pub enum Rulesets {
    Custom,
    Conway,
}
pub struct Ruleset {
    pub input_cells: InputCells,
    pub rules: Rulesets,
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
                 cases: Vec<(usize, CellState)>)
    {
        let key = (identity_state, for_state);

        self.data.insert(key, cases);
    }
    pub fn compute(&mut self,
               identity_state: CellState,
               input_states: Vec<(CellState, usize)>)
        -> CellState {

        let mut return_state = identity_state.clone();

        let mut key = (identity_state, CellState::Uninitialized);
        //loop through list of amounts of each state from neighbor states
        for &(ref state, ref amt) in input_states.iter() {
            //reuse identity_state
            key.1 = state.clone();

            match self.data.get(&key) {
                Some(vec) => {
                    for &(ref trigger_amt, ref result_state) in vec {
                        if trigger_amt == amt {
                            return_state = result_state.clone();
                        }
                    }
                },
                None => (),
            }
        }

        return_state
    }
}
