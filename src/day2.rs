#![allow(dead_code)]
fn parse_source(source: String) -> Vec<usize> {
    source
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect()
}

fn handle_opcode(
    code: Vec<usize>,
    position: usize,
    transform: impl Fn(&usize, &usize) -> usize
) -> Vec<usize> {
    let x = code.get(position + 1)
        .and_then(|&adress| code.get(adress))
        .unwrap();

    let y = code.get(position + 2)
        .and_then(|&adress| code.get(adress))
        .unwrap();

    code.get(position + 3)
        .map(|&adress| {
            let mut new_code = code.clone();
            new_code.remove(adress);
            new_code.insert(adress, transform(x, y));
            new_code
        })
        .unwrap()
}

fn multiply(code: Vec<usize>, position: usize) -> Vec<usize> {
    handle_opcode(code, position, |x, y| x * y)
}

fn add(code: Vec<usize>, position: usize) -> Vec<usize> {
    handle_opcode(code, position, |x, y| x + y)
}

fn handle_expression(code: Vec<usize>, position: usize) -> Vec<usize> {
    let opcode = code.get(position);

    match opcode {
        Some(1) => handle_expression(add(code, position), position + 4),
        Some(2) => handle_expression(multiply(code, position), position + 4),
        _ => code,
    }
}


fn run_code(code: Vec<usize>) -> Vec<usize>{
    handle_expression(code, 0)
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn correct_result() {
        let input = fs::read_to_string("input/day2.txt")
            .expect("oh no!");

        let code = parse_source(input);
        let code_result = run_code(code);
        let result = code_result.get(0).unwrap();
        assert_eq!(*result, 4090689);
    }

    #[test]
    fn correct_result_for_special_result() {
        let input = fs::read_to_string("input/day2.txt")
            .expect("oh no!");

        let mut code = parse_source(input);

        code.remove(1);
        code.insert(1, 77);
        code.remove(2);
        code.insert(2, 33);

        let code_result = run_code(code);
        let result = code_result.get(0).unwrap();
        assert_eq!(*result, 19690720);
    }
}
