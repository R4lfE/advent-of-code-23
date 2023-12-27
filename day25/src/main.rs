use std::{error::Error, fs};

fn part1(input: &str) -> usize {
    todo!()
}

fn part2(input: &str) -> usize {
    todo!()
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
        let input = r#""#;
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn part_2() {
        let input = r#""#;
        assert_eq!(part2(input), 0);
    }
}