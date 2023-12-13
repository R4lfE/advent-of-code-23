use std::{error::Error, fs, collections::HashMap};

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

fn recursion(springs: Vec<Spring>, groups: Vec<u64>, map: &mut HashMap<(Vec<Spring>, Vec<u64>), u64>) -> u64 {
    // search cache
    if let Some(arrangements) = map.get(&(springs.clone(), groups.clone())) {
        return *arrangements;
    }

    // termination condition
    if groups.is_empty() {
        // groups are handled, no more damaged springs left
        if springs.iter().all(|spring| *spring != Spring::Damaged) {
            map.insert((springs, groups), 1);
            return 1;
        // groups are handled, damaged springs remain
        } else {
            map.insert((springs, groups), 0);
            return 0;
        }
    // still have a group but no more springs, no arrangement found
    } else if springs.is_empty() {
        map.insert((springs, groups), 0);
        return 0;
    }

    // recurse
    match springs[0] {
        // skip if operational
        Spring::Operational => {
            let arrangements = recursion(springs.clone().into_iter().skip(1).collect(), groups.clone(), map);
            map.insert((springs, groups), arrangements);
            arrangements
        },
        // try place group if damaged
        Spring::Damaged => {
            let following_springs: Vec<Spring> = springs.clone().into_iter().skip(1).take(groups[0] as usize).collect();

            // can't place group, no solution possible
            if following_springs.iter().take(groups[0] as usize - 1).any(|spring| *spring == Spring::Operational) {
                map.insert((springs, groups), 0);
                0

            // otherwise check if last spring can separate the group
            } else if following_springs.len() == groups[0] as usize {
                match following_springs[groups[0] as usize - 1] {
                    // can't place group, no solution possible
                    Spring::Damaged => {
                        map.insert((springs, groups), 0);
                        0
                    },
                    // place group, continue to next subproblem
                    Spring::Operational | Spring::Unknown => {
                        let arrangements = recursion(
                            springs.clone().into_iter().skip(groups[0] as usize + 1).collect(), 
                            groups.clone().into_iter().skip(1).collect(), 
                            map
                        );
                        map.insert((springs, groups), arrangements);
                        arrangements
                    },
                }
            // place group, found arrangement
            } else if following_springs.len() == groups[0] as usize - 1 && groups.len() == 1 {
                map.insert((springs, groups), 1);
                1
            // can't place group
            } else {
                map.insert((springs, groups), 0);
                0
            }
        },
        // try place group or skip if unknown
        Spring::Unknown => {
            let following_springs: Vec<Spring> = springs.clone().into_iter().skip(1).take(groups[0] as usize).collect();

            // can't place group, place operational and continue to next subproblem
            if following_springs.iter().take(groups[0] as usize - 1).any(|spring| *spring == Spring::Operational) {
                let arrangements = recursion(springs.clone().into_iter().skip(1).collect(), groups.clone(), map);
                map.insert((springs, groups), arrangements);
                arrangements

            // otherwise check if last spring can separate the group
            } else if following_springs.len() == groups[0] as usize {
                match following_springs[groups[0] as usize - 1] {

                    // can't place group, place operational and continue to next subproblem
                    Spring::Damaged => {
                        let arrangements = recursion(springs.clone().into_iter().skip(1).collect(), groups.clone(), map);
                        map.insert((springs, groups), arrangements);
                        arrangements
                    },
                    // place group or don't place group, and continue to next subproblem
                    Spring::Operational | Spring::Unknown => {
                        let dont_place = recursion(springs.clone().into_iter().skip(1).collect(), groups.clone(), map);
                        let place = recursion(
                            springs.clone().into_iter().skip(groups[0] as usize + 1).collect(), 
                            groups.clone().into_iter().skip(1).collect(), 
                            map
                        );
                        map.insert((springs, groups), place + dont_place);

                        dont_place + place
                    },
                }
            // place group, found arrangement
            } else if following_springs.len() == groups[0] as usize - 1 && groups.len() == 1 {
                map.insert((springs, groups), 1);
                1
            // can't place group
            } else {
                map.insert((springs, groups), 0);
                0
            }
        },
    }
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            let springs: Vec<Spring> = split[0].chars().map(Spring::from).collect();
            let groups: Vec<u64> = split[1].split(',').map(|group| group.parse().unwrap()).collect();

            let mut map: HashMap<(Vec<Spring>, Vec<u64>), u64> = HashMap::new();
            recursion(springs, groups, &mut map)
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

            let mut map: HashMap<(Vec<Spring>, Vec<u64>), u64> = HashMap::new();
            recursion(springs, groups, &mut map)
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