use std::{error::Error, fs};

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(0, |acc, (i, game)| {
            let game = game.split(':').nth(1).unwrap();

            for reach in game.split(';') {
                for color in reach.split(',') {

                    let split: Vec<&str> = color.trim().split(' ').collect();
                    let num_cubes: u32 = split[0].parse().unwrap();

                    match split[1] {
                        "red" => if num_cubes > 12 {
                            return acc;
                        },
                        "green" => if num_cubes > 13 {
                            return acc;
                        },
                        "blue" => if num_cubes > 14 {
                            return acc;
                        },
                        _ => panic!("Wrong color.")
                    }
                }
            }

            acc + 1 + i as u32
        })
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .fold(0, |acc, game| {
            let mut reds = 0;
            let mut greens = 0;
            let mut blues = 0;

            let game = game.split(':').nth(1).unwrap();

            for reach in game.split(';') {
                for color in reach.split(',') {

                    let split: Vec<&str> = color.trim().split(' ').collect();
                    let num_cubes = split[0].parse().unwrap();

                    match split[1] {
                        "red" => reds = reds.max(num_cubes),
                        "green" => greens = greens.max(num_cubes),
                        "blue" => blues = blues.max(num_cubes),
                        _ => panic!("Wrong color.")
                    }
                }
            }

            acc + reds * greens * blues
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(&input), 2286);
    }
}