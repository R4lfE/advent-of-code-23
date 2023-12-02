use std::{error::Error, fs};

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .fold(0, |acc, line| {
            let digits: Vec<u32> = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect();

            match digits.first() {
                Some(first) => acc + 10 * first + digits.last().unwrap(),
                None => acc
            }
        })
}

fn part2(input: &str) -> u32 {
    let spelled = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];
    
    input
        .lines()
        .filter(|line| !line.is_empty())
        .fold(0, |acc, line| {
            let mut digits: Vec<(usize, u32)> = Vec::new();

            for digit in 1..=9 {
                if let Some(i) = line.find(char::from_digit(digit, 10).unwrap()) {
                    digits.push((i, digit));
                }
    
                if let Some(i) = line.rfind(char::from_digit(digit, 10).unwrap()) {
                    digits.push((i, digit));
                }
            }

            for (digit, spelling) in spelled.iter().enumerate() {
                if let Some(i) = line.find(spelling) {
                    digits.push((i, (digit + 1) as u32));
                }

                if let Some(i) = line.rfind(spelling) {
                    digits.push((i, (digit + 1) as u32));
                }
            }

            digits.sort_by(|a, b| a.0.cmp(&b.0));

            match digits.first() {
                Some(first) => acc + 10 * first.1 + digits.last().unwrap().1,
                None => acc
            }
        })
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
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn part_2() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        assert_eq!(part2(&input), 281);
    }
}