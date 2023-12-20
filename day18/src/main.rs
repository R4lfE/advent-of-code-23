use std::{error::Error, fs};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
use Direction::{Up, Down, Left, Right};

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' | '3' => Up,
            'D' | '1' => Down,
            'L' | '2' => Left,
            'R' | '0' => Right,
            _ => panic!("Error: Invalid character '{c}'.")
        }
    }
}

#[derive(Debug)]
struct Pattern {
    direction: Direction,
    meters: i64
}

impl Pattern {
    fn from_normal(str: &str) -> Self {
        let split: Vec<&str> = str.split_whitespace().collect();

        let direction = Direction::from(split[0].chars().next().unwrap());
        let meters: i64 = split[1].parse().unwrap();

        Pattern {
            direction,
            meters
        }
    }

    fn from_hex(str: &str) -> Self {
        let hex: &str = &str.split_whitespace().collect::<Vec<_>>()[2][2..=7];

        let direction = Direction::from(hex.chars().last().unwrap());
        let meters = i64::from_str_radix(&hex[..hex.len() - 1], 16).unwrap();

        Pattern {
            direction,
            meters
        }
    }
}

fn shoelace(dig_plan: Vec<Pattern>) -> i64 {
    // The coordinates are centered at the squares they fill.
    let mut coordinates: Vec<(i64, i64)> = Vec::new();

    // Determine the coordinates starting at (0, 0).
    for pattern in dig_plan.iter() {
        let previous = coordinates.last().unwrap_or(&(0, 0));
        coordinates.push(match pattern.direction {
            Up => (previous.0 - pattern.meters, previous.1),
            Down => (previous.0 + pattern.meters, previous.1),
            Left => (previous.0, previous.1 - pattern.meters),
            Right => (previous.0, previous.1 + pattern.meters),
        });
    }

    // Shoelace formula to compute the area of the polygon.
    let mut area: i64 = 0;
    for i in 0..coordinates.len() {
        area += coordinates[i].0 * coordinates[(i + 1) % coordinates.len()].1 - coordinates[(i + 1) % coordinates.len()].0 * coordinates[i].1;
    }
    area = (area / 2).abs();

    // Determine the orientation of the polygon.
    let mut current_direction = Direction::Up;
    let mut rights = 0;
    let mut lefts = 0;
    
    for direction in dig_plan.iter().map(|pattern| pattern.direction) {
        match current_direction {
            Up => {
                match direction {
                    Left => lefts += 1,
                    Right => rights += 1,
                    _ => ()
                }
            },
            Down => {
                match direction {
                    Left => rights += 1,
                    Right => lefts += 1,
                    _ => ()
                }
            },
            Left => {
                match direction {
                    Up => rights += 1,
                    Down => lefts += 1,
                    _ => ()
                }
            },
            Right => {
                match direction {
                    Up => lefts += 1,
                    Down => rights += 1,
                    _ => ()
                }
            }
        }

        current_direction = direction;
    }

    // We go straight ahead exactly as many times as the length of the border minus the number of times we turn.
    let path_len = dig_plan.iter().map(|pattern| pattern.meters).sum::<i64>();
    let halves = path_len - rights - lefts;

    // In clockwise orientation, places where we make a right don't count 3/4 of a square.
    // In clockwise orientation, places where we make a left don't count 1/4 of a square.
    // In counter clockwise orientation, this is swapped.
    area + halves / 2 + ((rights.max(lefts) as f64 * 3_f64 / 4_f64) + (lefts.min(rights) as f64 / 4_f64)) as i64
}

fn part1(input: &str) -> i64 {
    let dig_plan = input
        .lines()
        .map(Pattern::from_normal)
        .collect();
    shoelace(dig_plan)
}

fn part2(input: &str) -> i64 {
    let dig_plan = input
        .lines()
        .map(Pattern::from_hex)
        .collect();
    shoelace(dig_plan)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    
    dbg!(part1(&input));
    dbg!(part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"#;
        assert_eq!(part1(input), 62);
    }

    #[test]
    fn part_2() {
        let input = r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"#;
        assert_eq!(part2(input), 952408144115);
    }
}