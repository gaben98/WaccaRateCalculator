use std::io::stdin;

use itertools::Itertools;
use rust_decimal::prelude::*;

fn main() -> Result<(), String> {
    let mut rate_total: Decimal = dec!(0.0000);
    for i in 1..4 {
        println!("Enter song #{} (difficulty, score): ", i);
        let (difficulty, score) = {
            let mut input_string = String::new();
            stdin().read_line(&mut input_string)
                .map_err(|e| e.to_string())?;

            let atoms = input_string
                .split(",")
                .map(|s| s.replace("(", "").replace(")", "").trim().to_string());
            let (difficulty_string, score_string) = atoms.collect_tuple().expect("There should be exactly 2 inputs, separated by a comma!");

            (
                Decimal::from_str(&difficulty_string).map_err(|e| e.to_string())?,
                u32::from_str(&score_string).map_err(|e| e.to_string())?
            )
        };

        rate_total += calculate_rp(difficulty, score);
    }

    println!("Total rating: {}", rate_total);

    Ok(())
}


fn calculate_rp(difficulty: Decimal, score: u32) -> Decimal {
    let multiplier = match score {
        (0..990000) => dec!(3) + ((dec!(0.125) / dec!(5000)) * Decimal::from_u32(score - 950000).unwrap()),
        (990000..995000) => dec!(4) + ((dec!(0.01) / dec!(1000)) * Decimal::from_u32(score - 990000).unwrap()),
        (995000..) => dec!(4.05)
    };

    multiplier * difficulty
}
