use std::{time::Instant, io::Write};

use surd::Surd;

mod surd;
mod jett_implementation;

fn main() {
    let mut results: Vec<Vec<Surd>> = Vec::new();
    let start = Instant::now();
    for radicand in 1..1000000 {
        results.push(jett_implementation::simplify_surd(radicand));
    }
    let elapsed = start.elapsed();
    println!("{:#?}", elapsed);
    // Collect the results into a string.
    let mut result_string = String::new();
    for result in results {
        for surd in result {
            result_string.push_str(&surd.to_string());
            result_string.push_str("\n");
        }
    }
    // Write the results to a file.
    let mut file = std::fs::File::create("results.txt").expect("Could not create the output file");
    write!(file, "{}", result_string).expect("Could not write to the output file");
}
