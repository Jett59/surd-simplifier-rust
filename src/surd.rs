use std::fmt::Display;

pub enum DivisibilityRule {
    TrialAndError,
    Four,
    Nine,
    TwentyFive,
}

impl Display for DivisibilityRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DivisibilityRule::TrialAndError => write!(f, "Trial and Error"),
            DivisibilityRule::Four => write!(f, "4"),
            DivisibilityRule::Nine => write!(f, "9"),
            DivisibilityRule::TwentyFive => write!(f, "25"),
        }
    }
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
        // Add the working step if there is one.
        if self.working_step.previous_radicand != 0 {
            write!(f, "(because {} was divisible by {}, which I know because of the rule of {})", self.working_step.previous_radicand, self.working_step.square_number, self.working_step.divisibility_rule)?;
        }
        Ok(())
    }
}
