use std::{error::Error, fs};

fn view_galaxy(input: &str) -> Vec<&str> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .collect()
}

fn scale_galaxy(galaxies: Vec<&str>, scale: i64) -> Vec<(i64, i64)> {
    let empty_rows: Vec<usize> = galaxies.iter().enumerate().filter(|(_i, s)| !s.contains('#')).map(|(i, _s)| i).collect();
    let mut empty_cols: Vec<usize> = (0..galaxies[0].len()).collect();
    for galaxy_row in galaxies.iter() {
        for (j, galaxy) in galaxy_row.chars().enumerate() {
            if galaxy == '#' {
                empty_cols.retain(|value| *value != j);
            }
        }
    }

    let expanded: Vec<(i64, i64)> = galaxies
        .into_iter()
        .enumerate()
        .flat_map(|(i, row)| row
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(j, _)| (i as i64, j as i64))
            .collect::<Vec<_>>()
        ).map(|(i, j)| {
            (i + empty_rows.iter().filter(|row| **row < i as usize).count() as i64 * scale,
            j + empty_cols.iter().filter(|col| **col < j as usize).count() as i64 * scale)
        }).collect();

    expanded
}

pub fn solve(input: &str, scale: i64) -> i64 {
    let galaxies = view_galaxy(input);
    let galaxies = scale_galaxy(galaxies, scale - 1);

    galaxies
        .iter()
        .enumerate()
        .fold(0, |acc, (i, galaxy)| {
            galaxies
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, other)| {
                    (galaxy.0 - other.0).abs() + (galaxy.1 - other.1).abs()
                }).sum::<i64>()
            + acc
        }) / 2
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    
    dbg!(solve(&input, 2));
    dbg!(solve(&input, 1_000_000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";
        assert_eq!(solve(input, 2), 374);
    }

    #[test]
    fn part_2() {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";
        assert_eq!(solve(input, 100), 8410);
    }
}