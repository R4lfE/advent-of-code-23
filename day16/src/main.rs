use std::{error::Error, fs, collections::HashSet};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Eq, PartialEq)]
enum Object {
    Empty,
    MirrorS,
    MirrorBS,
    SplitterVertical,
    SplitterHorizontal
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::MirrorS,
            '\\' => Self::MirrorBS,
            '|' => Self::SplitterVertical,
            '-' => Self::SplitterHorizontal,
            _ => panic!("Error: Invalid character '{value}'.")
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}

struct Beam {
    i: usize,
    j: usize,
    direction: Direction,
    done: bool,
}

impl Beam {
    fn new() -> Self {
        Beam {
            i: 0,
            j: 0,
            direction: Direction::Right,
            done: false
        }
    }

    fn new_at(i: usize, j: usize, direction: Direction) -> Self {
        Beam {
            i,
            j,
            direction,
            done: false
        }
    }

    fn step(&mut self, contraption: &[Vec<Object>], path: &mut HashSet<(usize, usize, Direction)>) -> Option<Self> {

        if path.get(&(self.i, self.j, self.direction)).is_some() {
            self.done = true;
            return None;
        } else {
            path.insert((self.i, self.j, self.direction));
        }

        let mut new_beam: Option<Beam> = None;

        match contraption[self.i][self.j] {
            Object::Empty => (),
            Object::MirrorS => match self.direction {
                Direction::Up => self.direction = Direction::Right,
                Direction::Left => self.direction = Direction::Down,
                Direction::Right => self.direction = Direction::Up,
                Direction::Down => self.direction = Direction::Left,
            },
            Object::MirrorBS => match self.direction {
                Direction::Up => self.direction = Direction::Left,
                Direction::Left => self.direction = Direction::Up,
                Direction::Right => self.direction = Direction::Down,
                Direction::Down => self.direction = Direction::Right,
            },
            Object::SplitterVertical => match self.direction {
                Direction::Left | Direction::Right => { 
                    self.direction = Direction::Up;
                    new_beam = Some(Beam::new_at(self.i, self.j, Direction::Down));
                },
                _ => ()
            },
            Object::SplitterHorizontal => match self.direction {
                Direction::Up | Direction::Down => { 
                    self.direction = Direction::Left;
                    new_beam = Some(Beam::new_at(self.i, self.j, Direction::Right));
                },
                _ => ()
            },
        }

        match self.direction {
            Direction::Up => if self.i > 0 {
                self.i -= 1;
            } else {
                self.done = true;
            },
            Direction::Left => if self.j > 0 {
                self.j -= 1;
            } else {
                self.done = true;
            },
            Direction::Right => if self.j < contraption[self.i].len() - 1 {
                self.j += 1;
            } else {
                self.done = true;
            },
            Direction::Down => if self.i < contraption.len() - 1 {
                self.i += 1;
            } else {
                self.done = true;
            },
        }

        new_beam
    }
}

fn note_layout(input: &str) -> Vec<Vec<Object>> {
    input
        .lines()
        .map(|line| line
            .trim()
            .chars()
            .map(Object::from)
            .collect()
        ).collect()
}

fn find_energized(contraption: &[Vec<Object>], start_beam: Beam) -> usize {
    let mut beams = vec![start_beam];
    let mut path = HashSet::new();

    while !beams.is_empty() {

        let mut new_beams = Vec::new();
        for beam in beams.iter_mut().filter(|beam| !beam.done) {

            if let Some(new_beam) = beam.step(contraption, &mut path) {
                new_beams.push(new_beam);
            }
        }

        beams.retain(|beam| !beam.done);
        beams.extend(new_beams);
    }

    path.into_iter().map(|position| (position.0, position.1)).collect::<HashSet<_>>().len()
}

fn part1(input: &str) -> usize {
    let contraption = note_layout(input);
    find_energized(&contraption, Beam::new())
}

fn part2(input: &str) -> usize {
    let contraption = note_layout(input);
    let mut starting_beams = Vec::new();

    for (i, _) in contraption.iter().enumerate() {
        starting_beams.push(Beam::new_at(i, 0, Direction::Right));
        starting_beams.push(Beam::new_at(i, contraption[i].len() - 1, Direction::Left));
    }

    for j in 0..contraption[0].len() {
        starting_beams.push(Beam::new_at(0, j, Direction::Down));
        starting_beams.push(Beam::new_at(contraption.len() - 1, j, Direction::Up));
    }

    starting_beams.into_par_iter().map(|beam| find_energized(&contraption, beam)).max().unwrap()
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
        let input = r#".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|...."#;
        assert_eq!(part1(input), 46);
    }

    #[test]
    fn part_2() {
        let input = r#".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|...."#;
        assert_eq!(part2(input), 51);
    }
}