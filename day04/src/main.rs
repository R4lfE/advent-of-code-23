use std::{error::Error, fs, collections::HashSet};

fn parse_numbers<T>(card_part: &str) -> T 
where
    T: FromIterator<u32> {
    card_part
        .split_whitespace()
        .map(|value| value
            .parse()
            .unwrap()
        ).collect::<T>()
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| {
            let card: Vec<_> = line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split('|')
                .collect();

            let mut winners = parse_numbers::<HashSet<_>>(card[0]);
            let my_numbers = parse_numbers::<Vec<_>>(card[1]);

            winners.retain(|number| my_numbers.contains(number));

            if !winners.is_empty() {
                2_i32.pow(winners.len() as u32 - 1) as u32
            } else {
                0
            }
        }).sum()
}

fn part2(input: &str) -> u32 {
    let input: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .collect();

    let mut instance_counts = vec![1; input.len()];

    input
        .iter()
        .enumerate()
        .for_each(|(i, line)| {
            let card: Vec<_> = line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split('|')
                .collect();

            let mut winners = parse_numbers::<HashSet<_>>(card[0]);
            let my_numbers = parse_numbers::<Vec<_>>(card[1]);

            winners.retain(|number| my_numbers.contains(number));

            for j in (i + 1)..=(i + winners.len()) {
                if j < instance_counts.len() {
                    instance_counts[j] += instance_counts[i];
                }
            }
        });

    instance_counts.iter().sum()
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
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn part_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(input), 30);
    }
}