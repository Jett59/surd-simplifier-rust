use crate::surd::*;

fn calculate_step(radicand: u32, coefficient: u32) -> Option<Surd> {
    let last_two_digits = radicand % 100;
    // Rule of four.
    if last_two_digits % 4 == 0 {
        return Some(Surd {
            radicand: radicand / 4,
            coefficient: coefficient * 2,
            working_step: WorkingStep {
                previous_radicand: radicand,
                square_number: 4,
                divisibility_rule: DivisibilityRule::Four,
            },
        });
    }
    // Rule of nine.
    {
        let mut digit_sum: u32 = 0;
        let mut number = radicand;
        loop {
            while number != 0 {
                digit_sum += number % 10;
                number /= 10;
            }
            if digit_sum < 10 {
                break;
            } else {
                number = digit_sum;
                digit_sum = 0;
            }
        }
        if digit_sum == 9 {
            return Some(Surd {
                radicand: radicand / 9,
                coefficient: coefficient * 3,
                working_step: WorkingStep {
                    previous_radicand: radicand,
                    square_number: 9,
                    divisibility_rule: DivisibilityRule::Nine,
                },
            });
        }
    }
    // Rule of 25.
    if matches!(last_two_digits, 0 | 25 | 50 | 75) {
        return Some(Surd {
            radicand: radicand / 25,
            coefficient: coefficient * 5,
            working_step: WorkingStep {
                previous_radicand: radicand,
                square_number: 25,
                divisibility_rule: DivisibilityRule::TwentyFive,
            },
        });
    }
    // Rule of trial and error.
    // We only have to check 6n-1 and 6n+1. This is because we only need to check for divisibility by the squares of prime numbers, which are all of the form 6n-1 or 6n+1.
    for n in 1.. {
        let six_n = 6 * n;
        let mut prime_number = six_n - 1;
        let mut square_number = prime_number * prime_number;
        if radicand % square_number == 0 {
            return Some(Surd {
                radicand: radicand / square_number,
                coefficient: coefficient * prime_number,
                working_step: WorkingStep {
                    previous_radicand: radicand,
                    square_number,
                    divisibility_rule: DivisibilityRule::TrialAndError,
                },
            });
        }
        prime_number = six_n + 1;
        square_number = prime_number * prime_number;
        if radicand < square_number {
            break;
        }
        if radicand % square_number == 0 {
            return Some(Surd {
                radicand: radicand / square_number,
                coefficient: coefficient * prime_number,
                working_step: WorkingStep {
                    previous_radicand: radicand,
                    square_number,
                    divisibility_rule: DivisibilityRule::TrialAndError,
                },
            });
        }
    }
    None
}

pub fn simplify_surd(radicand: u32) -> Vec<Surd> {
    let mut surds: Vec<Surd> = Vec::with_capacity(10);
    surds.push(Surd {
        radicand: radicand,
        coefficient: 1,
        working_step: WorkingStep {
            previous_radicand: 0,
            square_number: 0,
            divisibility_rule: DivisibilityRule::TrialAndError,
        },
    });
    let mut current_radicand = radicand;
    let mut current_coefficient: u32 = 1;
    while let Some(surd) = calculate_step(current_radicand, current_coefficient) {
        current_radicand = surd.radicand;
        current_coefficient = surd.coefficient;
        surds.push(surd);
    }
    surds
}
