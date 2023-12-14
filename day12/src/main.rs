use std::{error::Error, fs};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
            _ => panic!("Error: Invalid spring found")
        }
    }
}

fn can_place(springs: &[Spring], groups: &[u64], i_spring: usize, i_group: usize) -> bool {
    // we cannot place the group
    // if the group cannot fit on the springs
    !(i_spring + 1 < i_group + groups[..i_group + 1].iter().sum::<u64>() as usize

    || (i_spring > groups[i_group] as usize 
    // or if the spring before the group is damaged
    && (springs[i_spring - groups[i_group] as usize] == Spring::Damaged

    // or if some spring in the group range is operational
    || springs[i_spring - groups[i_group] as usize + 1..i_spring].iter().any(|spring| *spring == Spring::Operational))))

    // otherwise we can now place the group starting at S[j - G[i] + 1]
}

fn tabulated(springs: Vec<Spring>, groups: Vec<u64>) -> u64 {
    let mut table = vec![vec![0_usize; springs.len() + 1]; groups.len() + 1];

    let first_damaged = springs.iter().position(|spring| *spring == Spring::Damaged).unwrap_or(springs.len());
    for j in 0..=first_damaged {
        table[0][j] = 1;
    }

    for i in 1..=groups.len() {
        for j in 1..=springs.len() {

            match springs[j - 1] {
                // no effect, give number of solutions of previous spring
                Spring::Operational => {
                    table[i][j] = table[i][j - 1];
                },
                // number of ways we can arrange previous groups up to current group placement
                // only if we can place the current group
                Spring::Damaged => {
                    table[i][j] = if can_place(&springs, &groups, j - 1, i - 1) {
                        let before_operational = if j > groups[i - 1] as usize {
                            1
                        } else {
                            0
                        };
                        table[i - 1][j - groups[i - 1] as usize - before_operational]
                    } else {
                        0
                    };
                },
                // sum of trying to place the group and considering this spring operational
                Spring::Unknown => {
                    table[i][j] = if can_place(&springs, &groups, j - 1, i - 1) {
                        let before_operational = if j > groups[i - 1] as usize {
                            1
                        } else {
                            0
                        };
                        table[i - 1][j - groups[i - 1] as usize - before_operational]
                    } else {
                        0
                    };
                    table[i][j] += table[i][j - 1];
                },
            }
        }
    }

    table[groups.len()][springs.len()] as u64
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            let springs: Vec<Spring> = split[0].chars().map(Spring::from).collect();
            let groups: Vec<u64> = split[1].split(',').map(|group| group.parse().unwrap()).collect();

            tabulated(springs, groups)
        }).sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            let springs: Vec<Spring> = [split[0]; 5].join("?").chars().map(Spring::from).collect();
            let groups: Vec<u64> = [split[1]; 5].join(",").split(',').map(|group| group.parse().unwrap()).collect();

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
        let input = "??? 1,1";
        assert_eq!(part1(input), 1);
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