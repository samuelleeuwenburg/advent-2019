#![allow(dead_code)]

fn is_six_digits_long(n: i64) -> bool {
    n >= 100_000 && n < 1_000_000
}

fn never_decreases(n: i64) -> bool {
    let mut last_char: Option<char> = None;
    let mut never_decreases = true;

    for c in n.to_string().chars() {
        match last_char {
            None => last_char = Some(c),
            Some(last_c) => {
                if c.to_digit(10).unwrap() < last_c.to_digit(10).unwrap() {
                    never_decreases = false;
                }
                last_char = Some(c)
            }
        }
    }

    never_decreases
}

fn has_correct_pairs(n: i64) -> bool {
    1234567890
        .to_string()
        .chars()
        .into_iter()
        .map(|digit| {
            n
                .to_string()
                .chars()
                .filter(|&c| c == digit)
                .collect::<Vec<char>>()
                .len()
        })
        .fold(false, |result, amount| {
            result || amount == 2 
        })
}

fn passwords(input: (i64, i64)) -> usize {
    let (floor, ceil) = input;
    let mut passwords: Vec<i64> = vec![];

    for n in floor..ceil + 1 {
        if is_six_digits_long(n) && never_decreases(n) && has_correct_pairs(n) {
            passwords.push(n)
        }
    }

    passwords.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_result() {
        let input = (193651, 659729);
        assert_eq!(passwords(input), 1102);
    }
}
