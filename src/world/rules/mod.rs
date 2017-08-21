pub mod input_cells;
use self::input_cells::InputCells;
use world::CellState;
use std::str::FromStr;
use std::collections::HashMap;

type RulesetType = HashMap<(CellState, CellState), Vec<(usize, CellState)>>;

#[derive(Clone)]
pub enum Rulesets {
    Custom,
    Conway,
    ConwayEasy,
    ConwayVeryEasy,
    Decay,
}
#[derive(Clone)]
pub struct Ruleset {
    pub input_cells: InputCells,
    pub rules: Rulesets,
}
impl FromStr for Rulesets {
    type Err = ();
    fn from_str(s: &str) -> Result<Rulesets, ()> {
        match s {
            "Conway" => Ok(Rulesets::Conway),
            "ConwayEasy" => Ok(Rulesets::ConwayEasy),
            "ConwayVeryEasy" => Ok(Rulesets::ConwayVeryEasy),
            "Decay" => Ok(Rulesets::Decay),
            _ => Err(()),
        }
    }
}
impl Rulesets {
    pub fn get_data(&self) -> DSLRuleset {
        use self::CellState::{Dead, Live};
        match *self {
            //Custom will use yet-formalized DSL that will be used for config files
            Rulesets::Custom => {
                let mut ruleset = DSLRuleset::new();
                ruleset.add_cases(Dead, Live, vec![(3, Live)]);
                ruleset
            },
            Rulesets::Conway => {
                let mut ruleset = DSLRuleset::new();
                ruleset.add_cases(Dead, Live, vec![(3, Live)]);
                ruleset.add_cases(Live, Live, vec![
                    (0, Dead), (1, Dead), (4, Dead), (5, Dead),
                    (6, Dead), (7, Dead), (8, Dead), (9, Dead),
                ]);
                ruleset
            },
            Rulesets::ConwayEasy => {
                let mut ruleset = DSLRuleset::new();
                ruleset.add_cases(Dead, Live, vec![(3, Live)]);
                ruleset.add_cases(Live, Live, vec![
                    (0, Dead), (1, Dead), (2, Live), (3, Live), 
                    (4, Dead), (5, Dead), (6, Dead), (7, Live), (8, Dead), 
                ]);
                ruleset
            },
            Rulesets::ConwayVeryEasy => {
                let mut ruleset = DSLRuleset::new();
                ruleset.add_cases(Dead, Live, vec![
                    (3, Live), (4, Live),
                ]);
                ruleset.add_cases(Live, Live, vec![
                    (0, Dead), (1, Dead), (4, Dead), (5, Dead),
                    (6, Dead), (7, Live), (8, Dead), (9, Dead),
                ]);
                ruleset
            },
            Rulesets::Decay => {
                let mut ruleset = DSLRuleset::new();
                ruleset.add_cases(Dead, Live, vec![(3, Live), (4, Live)]);
                ruleset.add_cases(Live, Live, vec![
                    (0, Dead), (1, Live), (2, Dead), (3, Dead), 
                    (4, Live), (5, Dead), (6, Dead), (7, Dead), (8, Dead), 
                ]);
                ruleset
            },
        }
    }
}
pub struct DSLRuleset {
    data: RulesetType,
}
impl DSLRuleset {
    pub fn new() -> Self {
        DSLRuleset {
            data: HashMap::new(),
        }
    }
    pub fn add_cases(&mut self,
                 identity_state: CellState,
                 for_state: CellState,
                 cases: Vec<(usize, CellState)>
        ) {
        let key = (identity_state, for_state);

        self.data.insert(key, cases);
    }
    pub fn compute(&mut self,
               identity_state: CellState,
               input_states: Vec<(CellState, usize)>
        ) -> CellState {

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
