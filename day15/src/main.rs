use std::{error::Error, fs};

fn read_sequence(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

fn hash(step: &str) -> usize {
    step.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn part1(input: &str) -> usize {
    let sequence = read_sequence(input);
    sequence
        .into_iter()
        .map(hash)
        .sum()
}

fn part2(input: &str) -> usize {
    let sequence = read_sequence(input);
    let mut hash_map: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    for step in sequence {
        if let Some(pos) = step.find('=') {
            let label = &step[..pos];
            let focal_length: usize = step[pos + 1..].parse().unwrap();
            let index = hash(label);

            let mut hit = false;
            for lens in hash_map[index].iter_mut() {

                if lens.0 == label {
                    lens.1 = focal_length;
                    hit = true;
                    break;
                }
            }

            if !hit {
                hash_map[index].push((label, focal_length));
            }

        } else {
            let label = &step[..step.len() - 1];
            let index = hash(label);
            hash_map[index].retain(|lens| lens.0 != label);
        }
    }

    hash_map
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, lens_box)| acc + (i + 1) * lens_box
            .into_iter()
            .enumerate()
            .fold(0, |acc, (j, lens)| acc + (j + 1) * lens.1))
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn part_2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(input), 145);
    }
}