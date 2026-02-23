#[derive(Clone, Debug)]
pub enum BioLoadFlag {
    Normal,
    Caution,  // corresponds to "!"
    Violation,
}

#[derive(Clone, Debug)]
pub enum SwarmMode {
    Normal,
    Caution,
    Rollback,
}

#[derive(Clone, Debug)]
pub struct SafetyState {
    pub k: f32,      // knowledge factor
    pub d: f32,      // normalized host energy demand
    pub dw: f32,     // psych-risk leverage
    pub bio_flag: BioLoadFlag,
    pub mode: SwarmMode,
}
