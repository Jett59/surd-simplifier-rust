use std::fmt::Display;

pub enum DivisibilityRule {
    TrialAndError,
    Four,
    Nine,
    TwentyFive,
}

pub struct WorkingStep {
    pub previous_radicand: u32,
    pub square_number: u32,
    pub divisibility_rule: DivisibilityRule,
}

pub struct Surd {
    pub radicand: u32,
    pub coefficient: u32,
    pub working_step: WorkingStep,
}

impl Display for Surd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.coefficient != 1 {
            write!(f, "{}", self.coefficient)?;
        }
        if self.radicand != 1 {
            write!(f, "sqrt({})", self.radicand)?;
        }
        if self.coefficient == 1 && self.radicand == 1 {
            write!(f, "1")?;
        }
        Ok(())
    }
}
