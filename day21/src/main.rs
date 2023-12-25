use std::{error::Error, fs, collections::HashSet};

fn adjacency(input: &str) -> (usize, Vec<Vec<usize>>) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().collect())
        .collect();

    let n = grid.len();

    let neighbors = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut list = vec![vec![]; n * n];

    let mut start = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &terrain) in row.iter().enumerate() {
            for neighbor in neighbors.iter().filter(|neighbor| {
                0 <= i as i32 + neighbor.1
                && i as i32 + neighbor.1 < n as i32
                && 0 <= j as i32 + neighbor.0
                && j as i32 + neighbor.0 < n as i32
            }) {
                if terrain != '#' && grid[(i as i32 + neighbor.1) as usize][(j as i32 + neighbor.0) as usize] != '#' {
                    list[i * n + j].push((i as i32 + neighbor.1) as usize * n + (j as i32 + neighbor.0) as usize);
                }
                if terrain == 'S' {
                    start = i * n + j;
                }
            }
        }
    }

    (start, list)
}

fn count_plots(steps: usize, start: usize, adjacency: &[Vec<usize>]) -> usize {
    let mut current: HashSet<usize> = HashSet::new();
    current.insert(start);

    for _ in 0..steps {
        let remove_nodes = current.clone();
        let mut new_nodes: HashSet<usize> = HashSet::new();
        for &nodes in current.iter() {
            for &neighbor in adjacency[nodes].iter() {
                new_nodes.insert(neighbor);
            }
        }
        for node in remove_nodes {
            current.remove(&node);
        }
        for node in new_nodes {
            current.insert(node);
        }
    }

    current.len()
}

fn part1(input: &str, steps: usize) -> usize {
    let (start, adjacency) = adjacency(input);
    count_plots(steps, start, &adjacency)
}

fn part2(input: &str, steps: usize) -> usize {
    // even and odd is set for this specific case, code not fitted to work in general (should be easy fix)
    let (start, adjacency) = adjacency(input);
    let n = (adjacency.len() as f32).sqrt() as usize;

    // odd mid side = even center = even corner
    let count_full_os_ecc = count_plots(n - 1, start, &adjacency);

    // even mid side = odd center = odd corner
    let count_full_es_occ = count_plots(n, start, &adjacency);

    let mut total_plots = count_full_es_occ;

    // count number of squares entered with an even number of steps and an odd number of steps from the side
    let grids_to_side = (steps - (n / 2 + 1)) / n;  // this is an odd number
    let mut even_grids = 0;
    let mut odd_grids = 0;
    for i in 0..grids_to_side {
        // works because steps and grids to side are both odd
        even_grids += (grids_to_side - i) / 2;
        odd_grids += (grids_to_side - i + 1) / 2;
    }
    even_grids *= 4;
    odd_grids *= 4;

    total_plots += even_grids * count_full_es_occ;
    total_plots += odd_grids * count_full_os_ecc;

    // mid sides: t, b, l, r
    let mid_sides = [n / 2, n * n - n / 2 - 1, (n - 1) * n / 2, n * (n + 1) / 2 - 1];

    // count plots on outer squares mid
    let steps_into_mid_sides = (steps - (n / 2 + 1)) % n;
    for mid_side in mid_sides {
        total_plots += count_plots(steps_into_mid_sides, mid_side, &adjacency);
    }

    // corners: tl, tr, bl, br
    let corners = [0, n - 1, n * (n - 1), n * n - 1];

    // count plots on outer squares corners
    let steps_into_inner_corner = steps_into_mid_sides + n / 2;
    let inner_corners = grids_to_side;

    let steps_into_outer_corner = (steps_into_mid_sides as i64 - ((n + 1) / 2) as i64).max(0) as usize;
    let outer_corners = grids_to_side + 1;

    for corner in corners {
        total_plots += count_plots(steps_into_inner_corner, corner, &adjacency) * inner_corners;
        total_plots += count_plots(steps_into_outer_corner, corner, &adjacency) * outer_corners;
    }
    
    total_plots
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    
    dbg!(part1(&input, 64));
    dbg!(part2(&input, 26501365));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#".......
        ....##.
        .##....
        ...S...
        .#..#..
        ..#..#.
        ......."#;
        dbg!(part1(input, 6));
    }

    #[test]
    fn part_2() {
        let input = r#".......
        ....##.
        .##....
        ...S...
        .#..#..
        ..#..#.
        ......."#;
        assert_eq!(part2(input, 44), 468);
    }
}