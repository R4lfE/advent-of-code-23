use std::{error::Error, fs};

fn part1(input: &str) -> u32 {
    let array: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut sum = 0;

    for i in 0..array.len() {

        let mut j_start = 0;
        while j_start < array.len() {

            let mut j_end = j_start;
            let mut num = String::new();
            while j_end < array[i].len() && array[i][j_end].is_numeric() {
                num.push(array[i][j_end]);
                j_end += 1;
            }

            if !num.is_empty() {
                for row in array.iter().take((i + 1).min(array.len() - 1) + 1).skip((i as i32 - 1).max(0) as usize) {
                    for c in row.iter().take((j_end).min(row.len() - 1) + 1).skip((j_start as i32 - 1).max(0) as usize) {
                        if !(c.is_numeric() || *c == '.') {
                            sum += num.parse::<u32>().unwrap();
                        }
                    }
                }
                j_start = j_end;
            } else {
                j_start += 1;
            }
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let input: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .collect();

    type Start = usize;
    type End = usize;
    type Value = u32;

    let indexed_numbers: Vec<Vec<(Start, End, Value)>> = input
        .iter()
        .map(|line| {
            let mut indexed_numbers = Vec::new();
            let chars: Vec<char> = line.chars().collect();

            let mut start = 0;
            while start < chars.len() {

                let mut end = start;
                let mut num = String::new();
                while end < chars.len() && chars[end].is_numeric() {
                    num.push(chars[end]);
                    end += 1;
                }

                if !num.is_empty() {
                    indexed_numbers.push((start, end - 1, num.parse().unwrap()));
                    start = end;
                } else {
                    start += 1;
                }
            }

            indexed_numbers
        }).collect();

    input
        .iter()
        .enumerate()
        .fold(0, |acc, (line_index, line)| {
            line
                .match_indices('*')
                .map(|(star_index, _star)| {
                    let mut adjacents: Vec<u32> = Vec::new();

                    for row in indexed_numbers.iter().take(line_index + 2).skip(line_index - 1) {
                        for indexed_number in row.iter() {

                            if indexed_number.0.max(star_index - 1) <= (indexed_number.1).min(star_index + 1) {
                                adjacents.push(indexed_number.2);
                            }
                        }
                    }

                    adjacents
                }).filter(|item| item.len() == 2)
                .fold(0, |acc, item| acc + item.iter().product::<u32>())
            + acc
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
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn part_2() {
        let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        100....100
        ..*.....*.
        ..10....20";
        assert_eq!(part2(input), 470835);
    }
}