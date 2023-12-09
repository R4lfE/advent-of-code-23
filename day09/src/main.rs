use std::{error::Error, fs};

fn read_report(input: &str, reverse: bool) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let readings = line
                .split_whitespace()
                .map(|num| num.parse().unwrap());

            match reverse {
                true => readings.rev().collect::<Vec<_>>(),
                false => readings.collect::<Vec<_>>()
            }
        }).collect()
}

fn solve(report: Vec<Vec<i32>>) -> i32 {
    report
        .into_iter()
        .fold(0, |acc, mut history| {

            let mut depth = 0;
            while history.iter().take(history.len() - depth).any(|v| *v != 0) {

                for i in 0..history.len() - 1 - depth {
                    history[i] = history[i + 1] - history[i];
                }
                depth += 1;
            }

            acc + history.iter().sum::<i32>()
        })
}

pub fn both(input: &str, reverse: bool) -> i32 {
    let report = read_report(input, reverse);
    solve(report)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    
    dbg!(both(&input, false));
    dbg!(both(&input, true));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(both(input, false), 114);
    }

    #[test]
    fn part_2() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(both(input, true), 2);
    }
}