use core::fmt;
use std::{error::Error, fs};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
    Empty
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Round,
            '#' => Self::Cube,
            '.' => Self::Empty,
            _ => panic!("Error: Invalid character '{value}'.")
        }
    }
}

impl fmt::Debug for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Round => write!(f, "O"),
            Self::Cube => write!(f, "#"),
            Self::Empty => write!(f, "."),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East
}

fn view_platform(input: String) -> Vec<Vec<Rock>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().map(Rock::from).collect())
        .collect()
}

fn slide(platform: &mut Vec<Vec<Rock>>, direction: Direction) {
    let mut north = 0;
    let mut west = 0;
    let mut south = 0;
    let mut east = 0;

    match direction {
        Direction::North => north = 1,
        Direction::West => west = 1,
        Direction::South => south = 1,
        Direction::East => east = 1,
    };

    if direction == Direction::North || direction == Direction::West {
        for i in north..platform.len() {
            for j in west..platform[i].len() {
                if platform[i][j] == Rock::Round {
                    if direction == Direction::North {
                        let mut row = i - 1;
                        while row > 0 && platform[row][j] == Rock::Empty {
                            row -= 1;
                        }

                        if platform[row][j] != Rock::Empty {
                            row += 1;
                        }

                        let swap = platform[i][j];
                        platform[i][j] = platform[row][j];
                        platform[row][j] = swap;
                    } else if direction == Direction::West {
                        let mut col = j - 1;
                        while col > 0 && platform[i][col] == Rock::Empty {
                            col -= 1;
                        }

                        if platform[i][col] != Rock::Empty {
                            col += 1;
                        }

                        platform[i].swap(j, col);
                    }
                }
            }
        }
    } else {
        for i in (0..platform.len() - south).rev() {
            for j in (0..platform[i].len() - east).rev() {
                if platform[i][j] == Rock::Round {
                    if direction == Direction::South {
                        let mut row = i + 1;
                        while row < platform.len() - 1 && platform[row][j] == Rock::Empty {
                            row += 1;
                        }

                        if platform[row][j] != Rock::Empty {
                            row -= 1;
                        }

                        let swap = platform[i][j];
                        platform[i][j] = platform[row][j];
                        platform[row][j] = swap;
                    } else if direction == Direction::East {
                        let mut col = j + 1;
                        while col < platform[i].len() - 1 && platform[i][col] == Rock::Empty {
                            col += 1;
                        }
    
                        if platform[i][col] != Rock::Empty {
                            col -= 1;
                        }
    
                        platform[i].swap(j, col);
                    }
                } 
            }
        }
    }
    
}

fn calculate_load(platform: &Vec<Vec<Rock>>) -> usize {
    platform
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|rock| **rock == Rock::Round).count() * (platform.len() - i))
        .sum()
}

fn part1(input: &str) -> usize {
    let mut platform = view_platform(input.to_string());
    slide(&mut platform, Direction::North);
    calculate_load(&platform)
}

fn part2(input: &str) -> usize {
    let cycle = [Direction::North, Direction::West, Direction::South, Direction::East];
    let mut platform = view_platform(input.to_string());
    
    let mut clones = Vec::new();

    while !clones.contains(&platform) {
        clones.push(platform.clone());
        for direction in cycle.iter() {
            slide(&mut platform, *direction);
        }
    }

    let pos = clones.iter().position(|clone| *clone == platform).unwrap();
    let cycle_len = clones.len() - pos;

    let reached_1b = (1_000_000_000_f64 - pos as f64) / cycle_len as f64;
    let final_platform = ((reached_1b - reached_1b.floor()) * cycle_len as f64).round() as usize + pos;

    calculate_load(&clones[final_platform])
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
        let input = "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....";
        assert_eq!(part1(input), 136);
    }

    #[test]
    fn part_2() {
        let input = "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....";
        assert_eq!(part2(input), 64);
    }
}