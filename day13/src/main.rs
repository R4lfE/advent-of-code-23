use std::{error::Error, fs};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Material {
    Ash,
    Rock
}

impl From<char> for Material {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Error: Invalid character '{value}'.")
        }
    }
}

impl Material {
    fn wipe_smudge(self) -> Self {
        match self {
            Self::Ash => Self::Rock,
            Self::Rock => Self::Ash
        }
    }
}

fn read_terrain(input: String) -> Vec<Vec<Vec<Material>>> {
    let mut all_terrain = Vec::new();
    let mut current_terrain = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            if !current_terrain.is_empty() {
                all_terrain.push(current_terrain);
                current_terrain = Vec::new();
            }
        } else {
            current_terrain.push(line.trim().chars().map(Material::from).collect())
        }
    }
    if !current_terrain.is_empty() {
        all_terrain.push(current_terrain);
    }

    all_terrain
}

fn check_mirror(materials: &Vec<Material>, start: usize) -> bool {
    let mut i = 0;

    while i <= start && start + 1 + i < materials.len() {
        if materials[start - i] != materials[start + 1 + i] {
            return false;
        }
        i += 1;
    }

    true
}

fn evaluate_terrain(terrain: &Vec<Vec<Material>>, skip: Option<(usize, bool)>) -> Option<(usize, bool)> {
    let mut possible_mirror: Vec<usize> = (0..terrain[0].len() - 1).collect();

    if let Some(skip) = skip {
        if !skip.1 {
            possible_mirror.remove(skip.0 - 1);
        }
    }

    // vertical mirror
    let mut row = 0;
    while row < terrain.len() && !possible_mirror.is_empty() {
        
        possible_mirror.retain(|col| check_mirror(&terrain[row], *col));
        row += 1;
    }

    if possible_mirror.len() == 1 {
        return Some((possible_mirror[0] + 1, false));
    }

    possible_mirror = (0..terrain.len() - 1).collect();

    if let Some(skip) = skip {
        if skip.1 {
            possible_mirror.remove(skip.0 / 100 - 1);
        }
    }

    // horizontal mirror
    let mut col = 0;
    while col < terrain[0].len() && !possible_mirror.is_empty() {
        possible_mirror.retain(|row| check_mirror(&terrain.iter().map(|row| row[col]).collect(), *row));
        col += 1;
    }

    if possible_mirror.len() == 1 {
        return Some(((possible_mirror[0] + 1) * 100, true));
    }

    None
}

fn part1(input: &str) -> usize {
    let all_terrain = read_terrain(input.to_string());
    all_terrain
        .into_iter()
        .map(|terrain| evaluate_terrain(&terrain, None).unwrap().0)
        .sum()
}

fn part2(input: &str) -> usize {
    let all_terrain = read_terrain(input.to_string());
    all_terrain
        .into_iter()
        .map(|mut terrain| {
            let with_smudge = evaluate_terrain(&terrain, None).unwrap();

            for i in 0..terrain.len() {
                for j in 0..terrain[0].len() {
                    terrain[i][j] = terrain[i][j].wipe_smudge();

                    if let Some(mirror) = evaluate_terrain(&terrain, Some(with_smudge)) {
                        return mirror.0;
                    }
                    terrain[i][j] = terrain[i][j].wipe_smudge();
                }
            }

            with_smudge.0
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
    fn mirror() {
        let materials: Vec<Material> = "#.##..###".chars().map(Material::from).collect();
        for (i, _) in materials.iter().enumerate() {
            dbg!(i, check_mirror(&materials, i));
        }
    }

    #[test]
    fn part_1() {
        let input = "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.";
        assert_eq!(part1(input), 5);

        let input = "#...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";
        assert_eq!(part1(input), 400);
    }

    #[test]
    fn part_2() {
        let input = "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.";
        assert_eq!(part2(input), 300);

        let input = "#...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";
        assert_eq!(part2(input), 100);
    }
}