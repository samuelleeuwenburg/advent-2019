#![allow(dead_code)]
use std::fs;

fn find_fuel(mass: f64) -> f64 {
    let fuel = (mass / 3.0).floor() - 2.0;

    if fuel > 0.0 {
        fuel + find_fuel(fuel)
    } else {
        0.0
    }
}

fn find_fuel_for_modules() -> f64 {
    let input = fs::read_to_string("input/day1.txt")
        .expect("oh no!");

    input
        .split_whitespace()
        .map(|module| {
            find_fuel(module.parse::<f64>().unwrap())
        })
        .fold(0.0, |acc, x| acc + x)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_result() {
        // assert_eq!(find_fuel_for_modules(), 3456641.0);
        assert_eq!(find_fuel_for_modules(), 5182078.0);
    }
}
