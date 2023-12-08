use std::collections::HashMap;
use num::Integer;

fn construct_map(nodes: Vec<&str>) -> HashMap<&str, (&str, &str)> {
    nodes
        .into_iter()
        .map(|node| {
            let splits: Vec<&str> = node.split_whitespace().collect();
            let source = splits[0];

            let mut left = splits[2].chars();
            left.next();
            left.next_back();
        
            let mut right = splits[3].chars();
            right.next_back();

            (source, (left.as_str(), right.as_str()))
        }).collect()
}

pub fn part1(input: &str) -> u32 {
    let mut input = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim());

    let directions: Vec<char> = input.next().unwrap().chars().collect();
    let map = construct_map(input.collect());

    let mut source = "AAA";
    let target = "ZZZ";

    let mut i = 0;
    while source != target {

        match directions[i % directions.len()] {
            'L' => source = map.get(source).unwrap().0,
            'R' => source = map.get(source).unwrap().1,
            _ => panic!("Error: Wrong direction found.")
        }

        i += 1;
    }

    i as u32
}

pub fn part2(input: &str) -> u64 {
    let mut input = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim());

    let directions: Vec<char> = input.next().unwrap().chars().collect();
    let map = construct_map(input.clone().collect());
    
    input
        .map(|node| node.split_whitespace().next().unwrap())
        .filter(|source| source.chars().nth(2).unwrap() == 'A')
        .map(|source| {
            let mut current = source;
            let mut i = 0;
            while current.chars().nth(2).unwrap() != 'Z' {

                match directions[i % directions.len()] {
                    'L' => current = map.get(current).unwrap().0,
                    'R' => current = map.get(current).unwrap().1,
                    _ => panic!("Error: Wrong direction found.")
                }

                i += 1;
            }

            i as u64
        }).fold(0, |acc, steps| {
            match acc {
                0 => steps,
                _ => Integer::lcm(&acc, &steps)
            }
        })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1() {
        let mut input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 2);

        input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn part_2() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        assert_eq!(part2(input), 6);
    }

    #[test]
    fn run_1() {
        let input = fs::read_to_string("input.txt").unwrap();
        dbg!(part1(&input));
    }

    #[test]
    fn run_2() {
        let input = fs::read_to_string("input.txt").unwrap();
        dbg!(part2(&input));
    }
}