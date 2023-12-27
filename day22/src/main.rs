use std::{error::Error, fs, collections::HashSet};

#[derive(Clone, Debug)]
enum Orientation {
    HorizontalX,
    HorizontalY,
    Vertical
}
use Orientation::*;

type Coordinate = (usize, usize, usize);

#[derive(Clone, Debug)]
struct Brick {
    id: usize,
    end_a: Coordinate,
    end_b: Coordinate,
    orientation: Orientation,
}

impl Brick {
    fn from_str(line: &str) -> Self {
        let ends: Vec<&str> = line.trim().split('~').collect();

        let end_a_vec: Vec<usize> = ends[0].split(',').map(|axis| axis.parse().unwrap()).collect();
        let end_a = (end_a_vec[0], end_a_vec[1], end_a_vec[2]);

        let end_b_vec: Vec<usize> = ends[1].split(',').map(|axis| axis.parse().unwrap()).collect();
        let end_b = (end_b_vec[0], end_b_vec[1], end_b_vec[2]);

        let orientation = if end_a.0 != end_b.0 {
            HorizontalX
        } else if end_a.1 != end_b.1 {
            HorizontalY
        } else {
            Vertical
        };

        Brick {
            id: 0,
            end_a,
            end_b,
            orientation
        }
    }
}

fn view_snapshot(input: &str) -> Vec<Brick> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Brick::from_str)
        .collect()
}

fn drop_bricks(mut bricks: Vec<Brick>, part: usize) -> (Vec<Brick>, Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    if part == 0 {
        // sort bricks by height
        bricks.sort_by(|a, b| {
            a.end_a.2.min(a.end_b.2).cmp(&b.end_a.2.min(b.end_b.2))
        });
        for (id, brick) in bricks.iter_mut().enumerate() {
            brick.id = id + 1;
        }
    }

    // id supports bricks in brick_supports[id] with artificial brick at the bottom
    let mut brick_supports: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len() + 1 + part];
    let mut brick_supported_by: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len() + 1 + part];

    // x width, y depth
    let width = bricks.iter().map(|brick| brick.end_a.0.max(brick.end_b.0)).max().unwrap() + 1;
    let depth = bricks.iter().map(|brick| brick.end_a.1.max(brick.end_b.1)).max().unwrap() + 1;
    let height = bricks.iter().map(|brick| brick.end_a.2.max(brick.end_b.2)).max().unwrap() + 1;

    // stack of bricks
    let mut stack: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; width]; depth]; height];

    // heights to manage dropping the bricks
    let mut heights: Vec<Vec<usize>> = vec![vec![0; width]; depth];

    // drop the bricks
    for brick in bricks.iter_mut() {
        match brick.orientation {
            HorizontalX => {
                // heights of supports directly under the brick
                let support_heights: Vec<&mut usize> = heights[brick.end_a.1]
                    .iter_mut()
                    .take(brick.end_a.0.max(brick.end_b.0) + 1)
                    .skip(brick.end_a.0.min(brick.end_b.0))
                    .collect();

                // drop the brick onto the support
                brick.end_a.2 = **support_heights.iter().max().unwrap() + 1;
                brick.end_b.2 = brick.end_a.2;

                // positions of highest supporting bricks
                let support_positions: Vec<usize> = support_heights
                    .iter()
                    .enumerate()
                    .filter(|(_, support_height)| ***support_height == brick.end_a.2 - 1)
                    .map(|(j, _)| j + brick.end_a.0.min(brick.end_b.0))
                    .collect();

                // handle ids of highest supporting bricks and say they support current brick
                for support_position in support_positions {
                    let support_id = stack[brick.end_a.2 - 1][brick.end_a.1][support_position];
                    brick_supports[support_id].insert(brick.id);
                    brick_supported_by[brick.id].insert(support_id);
                }

                // update the stack
                for block in stack[brick.end_a.2][brick.end_a.1]
                    .iter_mut()
                    .take(brick.end_a.0.max(brick.end_b.0) + 1)
                    .skip(brick.end_a.0.min(brick.end_b.0)) {
                        *block = brick.id;
                }
                
                // update support height to this brick
                for support_height in support_heights {
                    *support_height = brick.end_a.2;
                }
            },
            HorizontalY => {
                // heights of supports directly under the brick
                let support_heights: Vec<&mut usize> = heights
                    .iter_mut()
                    .take(brick.end_a.1.max(brick.end_b.1) + 1)
                    .skip(brick.end_a.1.min(brick.end_b.1))
                    .map(|support_height_row| &mut support_height_row[brick.end_a.0])
                    .collect();

                // drop the brick onto the support
                brick.end_a.2 = **support_heights.iter().max().unwrap() + 1;
                brick.end_b.2 = brick.end_a.2;

                // positions of highest supporting bricks
                let support_positions: Vec<usize> = support_heights
                    .iter()
                    .enumerate()
                    .filter(|(_, support_height)| ***support_height == brick.end_a.2 - 1)
                    .map(|(i, _)| i + brick.end_a.1.min(brick.end_b.1))
                    .collect();

                // handle ids of highest supporting bricks and say they support current brick
                for support_position in support_positions {
                    let support_id = stack[brick.end_a.2 - 1][support_position][brick.end_a.0];
                    brick_supports[support_id].insert(brick.id);
                    brick_supported_by[brick.id].insert(support_id);
                }

                // update the stack
                for block in stack[brick.end_a.2]
                    .iter_mut()
                    .take(brick.end_a.1.max(brick.end_b.1) + 1)
                    .skip(brick.end_a.1.min(brick.end_b.1)) {
                        block[brick.end_a.0] = brick.id;
                }

                // update support height to this brick
                for support_height in support_heights {
                    *support_height = brick.end_a.2;
                }
            },
            Vertical => {
                // height of support directly under the brick
                let support_height = &mut heights[brick.end_a.1][brick.end_a.0];

                // drop the brick onto the support
                let brick_height = brick.end_a.2.max(brick.end_b.2) - brick.end_a.2.min(brick.end_b.2);
                brick.end_a.2 = *support_height + 1;
                brick.end_b.2 = brick.end_a.2 + brick_height;

                // handle id of highest supporting brick and say it supports current brick
                let support_id = stack[brick.end_a.2 - 1][brick.end_a.1][brick.end_a.0];
                brick_supports[support_id].insert(brick.id);
                brick_supported_by[brick.id].insert(support_id);

                // update the stack
                for stack_level in stack.iter_mut().take(brick.end_b.2 + 1).skip(brick.end_a.2) {
                    stack_level[brick.end_a.1][brick.end_a.0] = brick.id;
                }

                // update support height to this brick
                *support_height = brick.end_b.2;
            },
        }
    }

    (bricks, brick_supports, brick_supported_by)
}

fn part1(input: &str) -> usize {
    let (bricks, brick_supports, brick_supported_by) = drop_bricks(view_snapshot(input), 0);

    // if all bricks supported by a brick are also supported by another brick then we can remove it
    bricks
        .into_iter()
        .filter(|brick| brick_supports[brick.id]
            .iter()
            .all(|&supported_brick| brick_supported_by[supported_brick].len() != 1)
        ).count()
}

fn part2(input: &str) -> usize {
    let (bricks, _, _) = drop_bricks(view_snapshot(input), 0);
    let heights: Vec<usize> = bricks.iter().map(|brick| brick.end_a.2).collect();

    // remove a brick and drop all others, check which bricks fell
    bricks
        .iter()
        .map(|brick| {
            let mut clone = bricks.clone();
            clone.remove(brick.id - 1);

            let (after_disintegration, _, _) = drop_bricks(clone, 1);
            after_disintegration
                .into_iter()
                .filter(|clone_brick| clone_brick.end_a.2 != heights[clone_brick.id - 1])
                .count()
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
    fn part_1() {
        let input = r#"1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9"#;
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn part_2() {
        let input = r#"1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9"#;
        assert_eq!(part2(input), 7);
    }
}