#![allow(dead_code)]
use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

type Path = Vec<Position>;
type Step = (Direction, i64);

const ORIGIN: Position = Position { x: 0, y: 0 };

fn take_step(path: Path, step: Step) -> Path {
    let &pos = path.last().clone().unwrap_or(&ORIGIN);
    let mut new_path = path;
    let (direction, length) = step;

    for i in 1..length + 1 {
        match direction {
            Direction::Right => new_path.push(Position { x: pos.x + i, ..pos }),
            Direction::Left => new_path.push(Position { x: pos.x - i, ..pos }),
            Direction::Up => new_path.push(Position { y: pos.y - i, ..pos }),
            Direction::Down => new_path.push(Position { y: pos.y + i, ..pos }),
        }
    }

    new_path
}


fn parse_string_into_steps(input: &str) -> Vec<Step> {
    input
        .split(",")
        .map(|step_str| {
            let mut direction_str = String::from(step_str.clone());
            let length = direction_str.split_off(1).parse::<i64>().unwrap();
            let direction = match direction_str.as_ref() {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("unknown direction"),
            };

            (direction, length)
        })
        .collect()
}

fn closest_distance_to_origin(crossings: &Vec<Position>) -> Option<i64> {
    crossings.into_iter().fold(None, |result, position| {
        let distance = position.x.abs() + position.y.abs();

        match result {
            None => Some(distance),
            Some(other_distance) => Some(cmp::min(distance, other_distance))
        }
    })
}

fn get_steps_to_crossing(position: Position, path: &Path) -> Option<i64> {
    let mut count = 0i64;
    let mut steps = None;

    for &pos in path {
        count = count + 1;

        if pos == position {
            steps = Some(count);
            break;
        }
    }

    steps
}

fn get_shortest_path(crossings: &Vec<Position>, path1: &Path, path2: &Path) -> Option<i64> {
    crossings.into_iter().fold(None, |shortest_path, &crossing| {
        let step1 = get_steps_to_crossing(crossing, &path1).unwrap();
        let step2 = get_steps_to_crossing(crossing, &path2).unwrap();
        let result = step1 + step2;

        match shortest_path {
            None => Some(result),
            Some(n) => if result < n { Some(result) } else { Some(n) },
        }
    })
}

fn wires(input: String) -> (Option<i64>, Option<i64>) {
    let paths: Vec<Path> = input
        .split_whitespace()
        .map(|str| {
            let wire = parse_string_into_steps(str);
            let mut path: Path = vec![];

            for step in wire {
                path = take_step(path, step);
            }

            path
        })
        .collect();

    let map = paths
        .clone()
        .into_iter()
        .enumerate()
        .fold(HashMap::new(), |mut map, (index, path)| {
            for position in path {
                match map.get(&position) {
                    Some(&(_, key_index)) => {
                        if key_index != index {
                            map.insert(position, (true, index));
                        }
                        ()
                    }
                    None => {
                        map.insert(position, (false, index));
                        ()
                    }
                };
            }

            map
        });

    let mut crossings: Vec<Position> = vec![];

    for (key, &val) in map.iter() {
        if val.0 == true {
            crossings.push(key.clone());
        }
    }

    let path1 = paths.get(0).unwrap();
    let path2 = paths.get(1).unwrap();

    let shortest_path = get_shortest_path(&crossings, &path1, &path2);
    let closest_distance = closest_distance_to_origin(&crossings);

    (shortest_path, closest_distance)
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn correct_result() {
        let input = fs::read_to_string("input/day3.txt").expect("oh no!");

        assert_eq!(wires(input), (Some(164012), Some(4981)));
    }
}
