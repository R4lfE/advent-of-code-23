use regex::Regex;
use std::{error::Error, fs};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Error: Invalid character '{value}'.")
        }
    }
}

fn can_place(springs: &[Spring], groups: &[usize], i_spring: usize, i_group: usize) -> bool {
    // we cannot place the group
    // if the group cannot fit on the springs
    !(i_spring + 1 < i_group + groups[..=i_group].iter().sum::<usize>()

    // or if the spring before the group is damaged
    || (i_spring >= groups[i_group]
        && (springs[i_spring - groups[i_group]] == Spring::Damaged)

    // or if some spring in the group range is operational
    || (i_spring + 1 >= groups[i_group]
        && springs[i_spring + 1 - groups[i_group]..i_spring].iter().any(|spring| *spring == Spring::Operational))))

    // otherwise we can now place the group starting at S[j - G[i] + 1]
}

fn tabulated(springs: Vec<Spring>, groups: Vec<usize>) -> usize {
    let mut table = vec![vec![0_usize; springs.len() + 1]; 2];

    // solution is valid when no groups or damaged springs exist
    let first_damaged = springs.iter().position(|spring| *spring == Spring::Damaged).unwrap_or(springs.len());
    for j in 0..=first_damaged {
        table[0][j] = 1;
    }

    for i in 1..=groups.len() {
        for j in 1..=springs.len() {

            // no effect, give number of solutions of previous sub problem
            if springs[j - 1] == Spring::Operational || springs[j - 1] == Spring::Unknown {
                table[i % 2][j] += table[i % 2][j - 1];
            }
            
            // number of ways we can arrange previous groups up to current group placement
            // only if we can place the current group on S[j - G[i] + 1]
            if (springs[j - 1] == Spring::Damaged || springs[j - 1] == Spring::Unknown)
                && can_place(&springs, &groups, j - 1, i - 1) {

                table[i % 2][j] += table[(i - 1) % 2][j - groups[i - 1] - (i > 1) as usize];
            }

            // unkown is the sum of trying to place the group and considering this spring operational
        }
        table[(i - 1) % 2] = vec![0; springs.len() + 1];
    }

    table[groups.len() % 2][springs.len()] as usize
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let re = Regex::new(r"\.+").unwrap();
            let split: Vec<&str> = line.split_whitespace().collect();

            let springs: Vec<Spring> = re
                .replace_all(split[0], ".")
                .trim_matches('.')
                .chars()
                .map(Spring::from)
                .collect();

            let groups: Vec<usize> = split[1].split(',').map(|group| group.parse().unwrap()).collect();

            tabulated(springs, groups)
        }).sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let re = Regex::new(r"\.+").unwrap();
            let split: Vec<&str> = line.split_whitespace().collect();
            let springs: Vec<Spring> = re
                .replace_all([split[0]; 5].join("?").as_str(), ".")
                .trim_matches('.')
                .chars()
                .map(Spring::from)
                .collect();

            let groups: Vec<usize> = [split[1]; 5].join(",").split(',').map(|group| group.parse().unwrap()).collect();

            tabulated(springs, groups)
        }).sum()
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
        let input = "???.### 1,1,3";
        assert_eq!(part1(input), 1);
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(part1(input), 4);
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(part1(input), 1);
        let input = "????.#...#... 4,1,1";
        assert_eq!(part1(input), 1);
        let input = "????.######..#####. 1,6,5";
        assert_eq!(part1(input), 4);
        let input = "?###???????? 3,2,1";
        assert_eq!(part1(input), 10);
    }

    #[test]
    fn part_2() {
        let input = "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";
        assert_eq!(part2(input), 525152);
    }
}