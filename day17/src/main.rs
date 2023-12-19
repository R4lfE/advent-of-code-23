use std::{error::Error, fs, collections::BinaryHeap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}
use Direction::{Up, Left, Right, Down};

#[derive(Clone, Copy, Debug, Eq)]
struct Node {
    cost: usize,
    i: usize,
    j: usize,
    d: Direction,
    c: usize
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j && self.d == other.d && self.c == other.c
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn new(cost: usize, i: usize, j: usize, d: Direction, c: usize) -> Node {
        Node { cost, i, j, d, c }
    }
}

fn read_traffic_patterns(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
        ).collect()
}

fn dijkstra(heat_loss_map: Vec<Vec<usize>>, min: usize, max: usize) -> usize {
    let n = heat_loss_map.len();
    let m = heat_loss_map[0].len();
    
    let mut dist = vec![vec![vec![vec![usize::MAX; m]; n]; max + 1]; 4];

    let start = Node::new(0, 0, 0, Right, 0);
    dist[start.d as usize][0][start.i][start.j] = 0;

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    heap.push(start);

    while let Some(current) = heap.pop() {
        if current.cost > dist[current.d as usize][current.c][current.i][current.j] {
            continue;
        }

        let mut possible_directions = Vec::new();

        // We need to keep going in the same direction until we reach min consecutive moves.
        if current.c < min && current.c != 0 {
            possible_directions.push(current.d);

        // We are now allowed to turn.
        } else {
            // Tests whether we are moving in the same direction or not and allows the neighboring directions accordingly.
            if current.j > (min - 1) * (current.d != Left) as usize && current.d != Right && (current.d != Left || current.c < max) {
                possible_directions.push(Left);
            }
            if current.j < m - 1 - (min - 1) * (current.d != Right) as usize && current.d != Left && (current.d != Right || current.c < max) {
                possible_directions.push(Right);
            }
            if current.i > (min - 1) * (current.d != Up) as usize && current.d != Down && (current.d != Up || current.c < max) {
                possible_directions.push(Up);
            }
            if current.i < n - 1 - (min - 1) * (current.d != Down) as usize && current.d != Up && (current.d != Down || current.c < max) {
                possible_directions.push(Down);
            }
        }

        for &pd in possible_directions.iter() {
            let (d_i, d_j) = match pd {
                Up => (current.i - 1, current.j),
                Left => (current.i, current.j - 1),
                Right => (current.i, current.j + 1),
                Down => (current.i + 1, current.j),
            };

            // If we move in the same direction then we have to increase c, otherwise reset c.
            let neighbor = Node::new(current.cost + heat_loss_map[d_i][d_j], d_i, d_j, pd, 1 + (pd == current.d) as usize * current.c);

            if neighbor.cost < dist[neighbor.d as usize][neighbor.c][neighbor.i][neighbor.j] {
                heap.push(neighbor);
                dist[neighbor.d as usize][neighbor.c][neighbor.i][neighbor.j] = neighbor.cost;
            }
        }
    }
    
    // Return smallest over all target states.
    dist.iter().map(|cons| cons.iter().map(|node| node[n-1][m-1]).min().unwrap()).min().unwrap()
}

fn part1(input: &str) -> usize {
    let heat_loss_map = read_traffic_patterns(input);

    // We have to make at least one move so we can set min to 1.
    dijkstra(heat_loss_map, 1, 3)
}

fn part2(input: &str) -> usize {
    let heat_loss_map = read_traffic_patterns(input);
    dijkstra(heat_loss_map, 4, 10)
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
        let input = r#"2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533"#;
        assert_eq!(part1(input), 102);
    }

    #[test]
    fn part_2() {
        let input = r#"2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533"#;
        assert_eq!(part2(input), 94);
    }
}