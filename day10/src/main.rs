use std::{error::Error, fs, collections::HashSet};

#[derive(Debug, PartialEq)]
enum Pipe {
    Horizontal,
    Vertical,
    NE,
    NW,
    SE,
    SW,
    Ground,
    Start
}

impl Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Error: Found wrong character.")
        }
    }

    fn get_neighbors(&self) -> Vec<(i32, i32)> {
        match self {
            Pipe::Horizontal => vec![(0, -1), (0, 1)],
            Pipe::Vertical => vec![(-1, 0), (1, 0)],
            Pipe::NE => vec![(-1, 0), (0, 1)],
            Pipe::NW => vec![(-1, 0), (0, -1)],
            Pipe::SE => vec![(1, 0), (0, 1)],
            Pipe::SW => vec![(1, 0), (0, -1)],
            Pipe::Ground => vec![],
            Pipe::Start => vec![(0, -1), (0, 1), (-1, 0), (1, 0)],
            _ => panic!("Error: Found wrong pipe.")
        }
    }

    fn possible(&self, other: &Self, (i, j): (i32, i32)) -> bool {
        match self {
            Pipe::Vertical => {
                if j == 0 {
                    if i == -1 {  
                        matches!(other,
                            Pipe::Vertical
                            | Pipe::SE
                            | Pipe::SW
                            | Pipe::Start
                        )
                    } else if i == 1 {  
                        matches!(other,
                            Pipe::Vertical
                            | Pipe::NE
                            | Pipe::NW
                            | Pipe::Start
                        )
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            Pipe::Horizontal => {
                if i == 0 {
                    if j == -1 {  
                        matches!(other,
                            Pipe::Horizontal
                            | Pipe::NE
                            | Pipe::SE
                            | Pipe::Start
                        )
                    } else if j == 1 {  
                        matches!(other,
                            Pipe::Horizontal
                            | Pipe::NW
                            | Pipe::SW
                            | Pipe::Start
                        )
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            Pipe::NE => {
                if i == -1 && j == 0 { 
                    matches!(other,
                        Pipe::Vertical
                        | Pipe::SE
                        | Pipe::SW
                        | Pipe::Start
                    )
                } else if i == 0 && j == 1 {
                    matches!(other,
                        Pipe::Horizontal
                        | Pipe::NW
                        | Pipe::SW
                        | Pipe::Start
                    )
                } else {
                    false
                }
            },
            Pipe::NW => {
                if i == -1 && j == 0 { 
                    matches!(other,
                        Pipe::Vertical
                        | Pipe::SE
                        | Pipe::SW
                        | Pipe::Start
                    )
                } else if i == 0 && j == -1 {
                    matches!(other,
                        Pipe::Horizontal
                        | Pipe::NE
                        | Pipe::SE
                        | Pipe::Start
                    )
                } else {
                    false
                }
            },
            Pipe::SE => {
                if i == 1 && j == 0 { 
                    matches!(other,
                        Pipe::Vertical
                        | Pipe::NE
                        | Pipe::NW
                        | Pipe::Start
                    )
                } else if i == 0 && j == 1 {
                    matches!(other,
                        Pipe::Horizontal
                        | Pipe::NW
                        | Pipe::SW
                        | Pipe::Start
                    )
                } else {
                    false
                }
            },
            Pipe::SW => {
                if i == 1 && j == 0 { 
                    matches!(other,
                        Pipe::Vertical
                        | Pipe::NE
                        | Pipe::NW
                        | Pipe::Start
                    )
                } else if i == 0 && j == -1 {
                    matches!(other,
                        Pipe::Horizontal
                        | Pipe::NE
                        | Pipe::SE
                        | Pipe::Start
                    )
                } else {
                    false
                }
            },
            Pipe::Start => {
                if j == 0 {
                    if i == -1 {  
                        matches!(other,
                            Pipe::Vertical
                            | Pipe::SE
                            | Pipe::SW
                        )
                    } else if i == 1 {  
                        matches!(other,
                            Pipe::Vertical
                            | Pipe::NE
                            | Pipe::NW
                        )
                    } else {
                        false
                    }
                } else if i == 0 {
                    if j == -1 {  
                        matches!(other,
                            Pipe::Horizontal
                            | Pipe::NE
                            | Pipe::SE
                        )
                    } else if j == 1 {  
                        matches!(other,
                            Pipe::Horizontal
                            | Pipe::NW
                            | Pipe::SW
                        )
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            _ => false
        }
    }
}

fn sketch_pipes(input: &str) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line
            .trim()
            .chars()
            .map(Pipe::from)
            .collect())
        .collect()
}

fn set_start(pipes: &mut Vec<Vec<Pipe>>, mut start: (i32, i32)) -> (i32, i32) {
    for (i, pipe_row) in pipes.iter().enumerate() {
        for (j, p) in pipe_row.iter().enumerate() {
            if *p == Pipe::Start {
                start = (i as i32, j as i32);
                break;
            }
        }
    }

    if 0 < start.1 
        && Pipe::Start.possible(&pipes[start.0 as usize][start.1 as usize - 1], (0, -1)) {
        if 0 < start.0 
            && Pipe::Start.possible(&pipes[start.0 as usize - 1][start.1 as usize], (-1, 0)) {
            pipes[start.0 as usize][start.1 as usize] = Pipe::NW;
        } else if start.0 < (pipes.len() - 1) as i32 
            && Pipe::Start.possible(&pipes[start.0 as usize + 1][start.1 as usize], (1, 0)) {
            pipes[start.0 as usize][start.1 as usize] = Pipe::SW;
        } else {
            pipes[start.0 as usize][start.1 as usize] = Pipe::Horizontal;
        }
    } else if start.1 < (pipes[start.0 as usize].len() - 1) as i32 
        && Pipe::Start.possible(&pipes[start.0 as usize][start.1 as usize + 1], (0, 1)) {
        if 0 < start.0 
            && Pipe::Start.possible(&pipes[start.0 as usize - 1][start.1 as usize], (-1, 0)) {
            pipes[start.0 as usize][start.1 as usize] = Pipe::NE;
        } else if start.0 < (pipes.len() - 1) as i32 
            && Pipe::Start.possible(&pipes[start.0 as usize + 1][start.1 as usize], (1, 0)) {
            pipes[start.0 as usize][start.1 as usize] = Pipe::SE;
        } else {
            pipes[start.0 as usize][start.1 as usize] = Pipe::Horizontal;
        }
    } else {
        pipes[start.0 as usize][start.1 as usize] = Pipe::Vertical;
    }

    start
}

#[derive(Clone, PartialEq)]
enum Dir {
    Up,
    Down,
    Right,
    Left
}

impl Dir {
    fn from(pipe: &Pipe, neihbor: &(i32, i32)) -> Self {
        match pipe {
            Pipe::Horizontal => if neihbor.1 == 1 { Dir::Right } else { Dir::Left },
            Pipe::Vertical => if neihbor.0 == 1 { Dir::Down } else { Dir::Up },
            Pipe::NE => if neihbor.0 == -1 { Dir::Up } else { Dir::Right },
            Pipe::NW => if neihbor.0 == -1 { Dir::Up } else { Dir::Left },
            Pipe::SE => if neihbor.0 == 1 { Dir::Down } else { Dir::Right },
            Pipe::SW => if neihbor.0 == 1 { Dir::Down } else { Dir::Left },
            _ => panic!("Error: Incorrect direction.")
        }
    }
}

struct PathNode {
    i: i32,
    j: i32,
    dir: Dir
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j
    }
}

fn find_path(pipes: &Vec<Vec<Pipe>>, start: (i32, i32)) -> Vec<PathNode> {
    let mut path = Vec::new();

    let mut prev = start;
    let mut current = start;

    let mut steps = 0;
    let mut dir = Dir::Up;
    while current != start || steps == 0 {
        let current_pipe = &pipes[current.0 as usize][current.1 as usize];
        let mut found = current;

        for neighbor in current_pipe.get_neighbors() {
            let other_pipe = &pipes[(current.0 + neighbor.0) as usize][(current.1 + neighbor.1) as usize];

            if (prev.0 != current.0 + neighbor.0
                || prev.1 != current.1 + neighbor.1)
                && current_pipe.possible(other_pipe, neighbor) {

                found = (current.0 + neighbor.0, current.1 + neighbor.1);
                dir = Dir::from(current_pipe, &neighbor);
                break;
            }
        }

        prev = current;
        current = found;
        path.push(PathNode {i: current.0, j: current.1, dir: dir.clone()});

        steps += 1;
    }

    path
}

pub fn part1(input: &str) -> i32 {
    let mut pipes = sketch_pipes(input);
    let mut start = (pipes.len() as i32, pipes[0].len() as i32);

    start = set_start(&mut pipes, start);
    let path = find_path(&pipes, start);

    path.len() as i32 / 2
}

pub fn part2(input: &str) -> i32 {
    let mut pipes = sketch_pipes(input);
    let mut start = (pipes.len() as i32, pipes[0].len() as i32);

    start = set_start(&mut pipes, start);
    let path = find_path(&pipes, start);

    let mut turn_counter: i32 = 0;
    let mut dir = path[0].dir.clone();
    for path_node in path.iter().skip(1) {
        match dir {
            Dir::Up => match path_node.dir {
                Dir::Right => turn_counter += 1,
                Dir::Left => turn_counter -= 1,
                _ => ()
            },
            Dir::Down => match path_node.dir {
                Dir::Right => turn_counter -= 1,
                Dir::Left => turn_counter += 1,
                _ => ()
            },
            Dir::Right => match path_node.dir {
                Dir::Up => turn_counter -= 1,
                Dir::Down => turn_counter += 1,
                _ => ()
            },
            Dir::Left => match path_node.dir {
                Dir::Up => turn_counter += 1,
                Dir::Down => turn_counter -= 1,
                _ => ()
            }
        }
        dir = path_node.dir.clone();
    }

    for i in 0..pipes.len() {
        for j in 0..pipes[i].len() {
            if !path.iter().any(|path_node| i == path_node.i as usize && j == path_node.j as usize) {
                pipes[i][j] = Pipe::Ground;
            }
        }
    }

    let sign = turn_counter.signum();
    let mut inside = HashSet::new();
    for path_node in path.iter() {
        match path_node.dir {
            Dir::Up => {
                let mut j = path_node.j + sign;
                while 0 <= j
                    && j < pipes[path_node.i as usize].len() as i32
                    && !path.contains(&PathNode{i: path_node.i, j, dir: dir.clone()}) {

                    pipes[path_node.i as usize][j as usize] = Pipe::Start;
                    inside.insert((path_node.i as usize, j as usize));
                    j += sign;
                }

                if (pipes[path_node.i as usize][path_node.j as usize] == Pipe::SE && sign == -1)
                    || (pipes[path_node.i as usize][path_node.j as usize] == Pipe::SW && sign == 1) {
                    let mut i = path_node.i - 1;
                    while 0 <= i
                        && i < pipes.len() as i32
                        && !path.contains(&PathNode{i, j: path_node.j, dir: dir.clone()}) {

                        pipes[i as usize][path_node.j as usize] = Pipe::Start;
                        inside.insert((i as usize, path_node.j as usize));
                        i -= 1;
                    }
                }
            },
            Dir::Down => {
                let mut j = path_node.j - sign;
                while 0 <= j
                    && j < pipes[path_node.i as usize].len() as i32
                    && !path.contains(&PathNode{i: path_node.i, j, dir: dir.clone()}) {

                    pipes[path_node.i as usize][j as usize] = Pipe::Start;
                    inside.insert((path_node.i as usize, j as usize));
                    j -= sign;
                }

                if (pipes[path_node.i as usize][path_node.j as usize] == Pipe::NE && sign == 1)
                    || (pipes[path_node.i as usize][path_node.j as usize] == Pipe::NW && sign == -1) {
                    let mut i = path_node.i + 1;
                    while 0 <= i
                        && i < pipes.len() as i32
                        && !path.contains(&PathNode{i, j: path_node.j, dir: dir.clone()}) {

                        pipes[i as usize][path_node.j as usize] = Pipe::Start;
                        inside.insert((i as usize, path_node.j as usize));
                        i += 1;
                    }
                }
            },
            Dir::Right => {
                let mut i = path_node.i + sign;
                while 0 <= i
                    && i < pipes.len() as i32
                    && !path.contains(&PathNode{i, j: path_node.j, dir: dir.clone()}) {

                    pipes[i as usize][path_node.j as usize] = Pipe::Start;
                    inside.insert((i as usize, path_node.j as usize));
                    i += sign;
                }

                if (pipes[path_node.i as usize][path_node.j as usize] == Pipe::NW && sign == 1)
                    || (pipes[path_node.i as usize][path_node.j as usize] == Pipe::SW && sign == -1) {
                    let mut j = path_node.j + 1;
                    while 0 <= j
                        && j < pipes[i as usize].len() as i32
                        && !path.contains(&PathNode{i: path_node.i, j, dir: dir.clone()}) {

                        pipes[path_node.i as usize][j as usize] = Pipe::Start;
                        inside.insert((path_node.i as usize, j as usize));
                        j += 1;
                    }
                }
            },
            Dir::Left => {
                let mut i = path_node.i - sign;
                while 0 <= i
                    && i < pipes.len() as i32
                    && !path.contains(&PathNode{i, j: path_node.j, dir: dir.clone()}) {

                    pipes[i as usize][path_node.j as usize] = Pipe::Start;
                    inside.insert((i as usize, path_node.j as usize));
                    i -= sign;
                }

                if (pipes[path_node.i as usize][path_node.j as usize] == Pipe::NE && sign == -1)
                    || (pipes[path_node.i as usize][path_node.j as usize] == Pipe::SE && sign == 1) {
                    let mut j = path_node.j - 1;
                    while 0 <= j
                        && j < pipes[i as usize].len() as i32
                        && !path.contains(&PathNode{i: path_node.i, j, dir: dir.clone()}) {

                        pipes[path_node.i as usize][j as usize] = Pipe::Start;
                        inside.insert((path_node.i as usize, j as usize));
                        j -= 1;
                    }
                }
            }
        }
    }

    inside.len() as i32
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
        let input = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";
        assert_eq!(part1(input), 4);

        let input = "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn part_2() {
        let input = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";
        assert_eq!(part2(input), 4);

        let input = ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";
        assert_eq!(part2(input), 8);

        let input = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part2(input), 10);
    }
}