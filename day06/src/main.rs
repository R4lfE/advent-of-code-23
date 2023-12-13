use std::{error::Error, fs};

fn part1(input: &str) -> u32 {
    let input: Vec<Vec<f32>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line
                .trim()
                .split(':')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect()
        }).collect();

    let mut win_mul = 1;
    for i in 0..input[0].len() {
        let mut win_count = 0;
        let t = input[0][i];
        let d = input[1][i];
        for j in 0..t as usize {
            if (t - j as f32) * j as f32 > d {
                win_count += 1;
            }
        }
        win_mul *= win_count;
    }

    win_mul
}

fn part2(input: &str) -> u64 {
    let input: Vec<u64> = input
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            line
                .trim()
                .split(':')
                .nth(1)
                .unwrap()
                .replace(' ', "")
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect::<Vec<u64>>()
        }).collect();

    let t = input[0];
    let d = input[1];

    let mut left = 0;
    let mut right = t;
    while left != right {
        let mid = left + (right - left) / 2;
        if (t - mid) * mid > d {
            right = mid;
        } else {
            left = mid;
        }
        if left == right - 1 {
            left = right
        }
    }
    let first = left;

    left = 0;
    right = t;
    while left != right {
        let mid = left + (right - left) / 2;
        if (t - mid) * mid > d {
            left = mid;
        } else {
            right = mid;
        }
        if left == right - 1 {
            right = left
        }
    }
    let last = left;
    
    last - first + 1
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
        let input = "Time:      7  15   30
        Distance:  9  40  200";
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn part_2() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";
        assert_eq!(part2(input), 71503);
    }
}