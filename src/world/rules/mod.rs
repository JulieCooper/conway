pub mod input_cells;
use self::input_cells::InputCells;
use world::CellState;
use std::str::FromStr;
use std::collections::HashMap;

//type RulesetType = HashMap<(CellState, CellState), Vec<(usize, CellState)>>;
type RulesetType = HashMap<(usize, usize), Vec<(usize/*amount*/, usize)>>;

#[derive(Clone)]
pub enum Rulesets {
    Custom,
    Conway,
    Decay,
    Wire,
    Bounce,
//    ConwayEasy,
//    ConwayVeryEasy,
//    Decay,
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
            "Decay" => Ok(Rulesets::Decay),
            "Wire" => Ok(Rulesets::Wire),
            "Bounce" => Ok(Rulesets::Bounce),
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
                //ruleset.add_cases(, 0, vec![(3, Live)]);
                ruleset
            },
            Rulesets::Conway => {
                let mut ruleset = DSLRuleset::new();
                ruleset.add_states(vec!["dead", "live"]);
                ruleset.add_cases("dead", "live", vec![(3, "live")]);
                ruleset.add_cases("live", "live", vec![
                    (0, "dead"), (1, "dead"), (4, "dead"), (5, "dead"),
                    (6, "dead"), (7, "dead"), (8, "dead"), (9, "dead"),
                ]);
                ruleset
            },
            Rulesets::Decay => {
                let mut ruleset = DSLRuleset::new();
                ruleset.add_states(vec!["dead", "live"]);
                ruleset.add_cases("dead", "live", vec![(3, "live"), (4, "live")]);
                ruleset.add_cases("live", "live", vec![
                    (0, "dead"), (1, "live"), (2, "dead"), (3, "dead"),
                    (4, "live"), (5, "dead"), (6, "dead"), (7, "dead"), (8, "dead"),
                ]);
                ruleset
            },
            Rulesets::Wire => {
                let mut ruleset = DSLRuleset::new();
                ruleset.add_states(vec!["empty", "head", "tail", "conductor"]);
                //ruleset.add_cases("empty", "none", vec![(0, "conductor")]);
                ruleset.add_cases("head", "none", vec![(0, "tail")]);
                ruleset.add_cases("tail", "none", vec![(0, "conductor")]);
                ruleset.add_cases("conductor", "head", vec![
                    (1, "head"), (2, "head")
                ]);
                ruleset
            },
            Rulesets::Bounce => {
                let mut ruleset = DSLRuleset::new();
                ruleset.add_states(vec![""]);

                ruleset
            },
        }
    }
}
pub struct DSLRuleset {
    labels: HashMap<&'static str, usize>,
    data: RulesetType,
}
impl DSLRuleset {
    pub fn new() -> Self {
        let mut labels = HashMap::new();
        labels.insert("none", 9998);
        DSLRuleset {
            data: HashMap::new(),
            labels: labels,
        }
    }
    pub fn add_states(&mut self, new_states: Vec<&'static str>) {
        for (index, state) in new_states.iter().enumerate() {
            //let i = self.labels.len();
            self.labels.insert(state, index);
        }
    }
    pub fn add_cases(&mut self,
                 identity_state: &'static str,
                 for_state: &'static str,
                 mut cases: Vec<(usize, &'static str)>
        ) {

        let identity_state = {
            self.labels.get(identity_state)
        }.expect("invalid identity state");
        let for_state = {
            self.labels.get(for_state)
        }.expect("invalid for state");

        let key = (*identity_state, *for_state); // might need to clone

        let cases = cases.iter().map(|case| {
            (case.0,
             *self.labels.get(case.1).expect("invalid state")
            )
        }).collect::<Vec<(usize, usize)>>();

        //println!("is: {}, fs: {}", identity_state, for_state);
        //println!("{:?}", cases);

        self.data.insert(key, cases);
    }
    pub fn compute(&mut self,
               identity_state: usize,
               input_states: &Vec<(usize, usize)>
        ) -> usize {

        let mut return_state = identity_state.clone();

        let mut key = (identity_state, 9999);
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
