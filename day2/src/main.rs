use std::{error::Error, fs};

fn part1(input: &Vec<&str>) -> u32 {
    input
        .iter()
        .enumerate()
        .fold(0, |acc, (i, game)| {
            let game = game.split(':').nth(1).unwrap();

            let reaches: Vec<&str> = game.split(';').collect();
            for reach in reaches {

                let colors: Vec<&str> = reach.split(',').collect();
                for color in colors {

                    let split: Vec<&str> = color.split(' ').collect();
                    let cubes = split[1].parse::<u32>().unwrap();

                    match split[2] {
                        "red" => if cubes > 12 {
                            return acc;
                        },
                        "green" => if cubes > 13 {
                            return acc;
                        },
                        "blue" => if cubes > 14 {
                            return acc;
                        },
                        _ => panic!("Wrong color.")
                    }
                }
            }

            acc + 1 + i as u32
        })
}

fn part2(input: &Vec<&str>) -> u32 {
    input
        .iter()
        .fold(0, |acc, game| {
            let mut reds = 0;
            let mut greens = 0;
            let mut blues = 0;

            let game = game.split(':').nth(1).unwrap();

            let reaches: Vec<&str> = game.split(';').collect();
            for reach in reaches {

                let colors: Vec<&str> = reach.split(',').collect();
                for color in colors {

                    let split: Vec<&str> = color.split(' ').collect();
                    let num_cubes = split[1].parse().unwrap();

                    match split[2] {
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
    let binding = fs::read_to_string("input.txt")?;
    let input: Vec<&str> = binding
        .split("\r\n")
        .collect();
    
    dbg!(part1(&input));
    dbg!(part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input: Vec<&str> = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .split("\n")
        .collect();
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn part_2() {
        let input: Vec<&str> = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .split("\n")
        .collect();
        assert_eq!(part2(&input), 2286);
    }
}