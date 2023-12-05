use std::{error::Error, fs};

fn part1(input: &str) -> i64 {
    let mut input = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim());

    let seeds: Vec<i64> = input
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();

    let mut maps: Vec<Vec<Vec<i64>>> = Vec::with_capacity(7);
    let mut map: Vec<Vec<i64>> = Vec::with_capacity(3);

    input.next();
    while let Some(line) = input.next() {

        let numbers: Vec<&str> = line
            .split_whitespace()
            .collect();

        if numbers.len() == 3 {
            let numbers: Vec<i64> = numbers
                .iter()
                .map(|num| num.parse().unwrap())
                .collect();

            map.push(numbers);

        } else {
            maps.push(map.clone());
            map.clear();
        }
    }
    maps.push(map);

    let mut mapped: Vec<i64> = seeds;
    for i in 0..7 {
        mapped = mapped
            .iter()
            .map(|seed| {
                for range in maps[i].iter() {
                    if range[1] <= *seed && *seed <= range[1] + range[2] {
                        return *seed - range[1] + range[0];
                    }
                }
                *seed
            })
            .collect();
    }

    *mapped.iter().min().unwrap()
}

#[derive(Clone, Debug, Eq, Ord)]
enum Boundary {
    Seed(i64),
    LeftMap(i64, i64),
    RightMap(i64)
}

impl PartialEq for Boundary {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Seed(a), Self::LeftMap(b, _)) 
            | (Self::Seed(a), Self::RightMap(b))
            | (Self::LeftMap(a, _), Self::Seed(b))
            | (Self::RightMap(a), Self::Seed(b)) => a == b,
            _ => false
        }
    }
}

impl PartialOrd for Boundary {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Seed(a), Self::Seed(b))
            | (Self::LeftMap(a, _), Self::LeftMap(b, _))
            | (Self::LeftMap(a, _), Self::RightMap(b))
            | (Self::RightMap(a), Self::LeftMap(b, _))
            | (Self::RightMap(a), Self::RightMap(b)) => a.partial_cmp(b),

            (Self::Seed(a), Self::LeftMap(b, _)) 
            | (Self::Seed(a), Self::RightMap(b)) => if a == b {
                Some(std::cmp::Ordering::Less)
            } else {
                a.partial_cmp(b)
            },

            (Self::LeftMap(a, _), Self::Seed(b))
            | (Self::RightMap(a), Self::Seed(b)) => if a == b {
                Some(std::cmp::Ordering::Greater)
            } else {
                a.partial_cmp(b)
            }
        }
    }
}

fn part2(input: &str) -> i64 {
    let mut input = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim());

    // collect all seed range boundaries
    let mut seed_boundaries: Vec<Boundary> = input
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .chunks(2)
        .flat_map(|chunk| {
            Vec::from([
                Boundary::Seed(chunk[0]),
                Boundary::Seed(chunk[0] + chunk[1] - 1)
            ])
        })
        .collect();

    // collect all map range boundaries
    let mut maps: Vec<Vec<Boundary>> = Vec::with_capacity(7);
    while let Some(line) = input.next() {
        let numbers: Vec<&str> = line
            .split_whitespace()
            .collect();

        // line is a map numbers row
        if numbers.len() == 3 {
            let left = numbers[1].parse().unwrap();
            let right = left + numbers[2].parse::<i64>().unwrap() - 1;
            let mapped_value = numbers[0].parse::<i64>().unwrap() - left;

            let mut current_map = maps.pop().unwrap();
            current_map.push(Boundary::LeftMap(left, mapped_value));
            current_map.push(Boundary::RightMap(right));
            maps.push(current_map);

        // line is a map title row
        } else {
            maps.push(Vec::new());
        }
    }
    
    // map the ranges for each mapping type
    for i in 0..7 {

        // merge all boundaries together and sort by value
        seed_boundaries.extend(maps[i].clone());
        seed_boundaries.sort();

        // prepare to construct new seed boundaries based on the mapping
        let mut boundaries = seed_boundaries.clone();
        seed_boundaries.clear();

        // remember range states
        let mut seed_open = false;
        let mut map_open = false;

        // remember left seed boundary event and open map modifier
        let mut left_seed_boundary = 0;
        let mut map_modifier = 0;

        // handle each boundary event
        for boundary in boundaries.iter_mut() {
            match boundary {
                // found a seed boundary
                Boundary::Seed(right_seed_boundary) => {
                    // if we found the right boundary of the seed range
                    if seed_open {

                        // if map range is open, include left_seed_boundary event and modify
                        if map_open {
                            seed_boundaries.push(Boundary::Seed(left_seed_boundary + map_modifier));
                            seed_boundaries.push(Boundary::Seed(*right_seed_boundary + map_modifier));

                        // if map range is closed, include left_seed_boundary event without modifying
                        } else {
                            seed_boundaries.push(Boundary::Seed(left_seed_boundary));
                            seed_boundaries.push(Boundary::Seed(*right_seed_boundary));
                        }

                    // if we found the left boundary of the seed range, remember it
                    } else {
                        left_seed_boundary = *right_seed_boundary;
                    }

                    // flip the seed range state
                    seed_open = !seed_open;
                },
                // found left boundary of the map range
                Boundary::LeftMap(right_seed_boundary, mapped_value) => {

                    // if seed range is open, we need to split the range excluding current value without modifying
                    if seed_open {
                        seed_boundaries.push(Boundary::Seed(left_seed_boundary));
                        seed_boundaries.push(Boundary::Seed(*right_seed_boundary - 1));

                        // remember the cutoff
                        left_seed_boundary = *right_seed_boundary;
                    }

                    // remember the map modifier
                    map_modifier = *mapped_value;

                    // map range opened
                    map_open = true;
                },
                // found right boundary of the map range
                Boundary::RightMap(right_seed_boundary) => {

                    // if seed range is open we, need to split the range including current value and modify
                    if seed_open {
                        seed_boundaries.push(Boundary::Seed(left_seed_boundary + map_modifier));
                        seed_boundaries.push(Boundary::Seed(*right_seed_boundary + map_modifier));

                        // remember the cutoff
                        left_seed_boundary = *right_seed_boundary + 1;
                    }

                    // forget the map modifier
                    map_modifier = 0;

                    // map range closed
                    map_open = false;
                }
            }
        }
    }
    
    // return the minimum over all mapped ranges
    if let Some(boundary) = seed_boundaries.iter().min() {
        match boundary {
            Boundary::Seed(min) => *min,
            _ => panic!("Error: Boundary is nod a seed.")
        }
    } else {
        panic!("Error: No boundaries found.")
    }
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
        let input = "seeds: 79 14 55 13
        seed-to-soil map:
        50 98 2
        52 50 48
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        water-to-light map:
        88 18 7
        18 25 70
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        temperature-to-humidity map:
        0 69 1
        1 0 69
        humidity-to-location map:
        60 56 37
        56 93 4";
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn part_2() {
        let input = "seeds: 79 14 55 13
        seed-to-soil map:
        50 98 2
        52 50 48
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        water-to-light map:
        88 18 7
        18 25 70
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        temperature-to-humidity map:
        0 69 1
        1 0 69
        humidity-to-location map:
        60 56 37
        56 93 4";
        assert_eq!(part2(&input), 46);
    }
}