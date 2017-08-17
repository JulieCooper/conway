#[derive(Debug)]
pub enum StepError {
    Reason1,
    Reason2,
    Reason3,
}
#[derive(Debug)]
pub struct StepResult {
    pub steps: u64,
    pub updated_cells: u64,
}
