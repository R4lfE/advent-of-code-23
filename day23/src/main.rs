use std::{error::Error, fs, collections::{BinaryHeap, HashMap, VecDeque, HashSet}};

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
use Direction::*;

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
            _ => panic!("Error: Invalid character '{c}'.")
        }
    }
}

impl Direction {
    fn neighbor(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Up => (position.0 - 1, position.1),
            Down => (position.0 + 1, position.1),
            Left => (position.0, position.1 - 1),
            Right => (position.0, position.1 + 1)
        }
    }
}

#[derive(PartialEq, Eq)]
enum Terrain {
    Path,
    Forest,
    Slope(Direction)
}
use Terrain::*;

impl From<char> for Terrain {
    fn from(c: char) -> Self {
        match c {
            '.' => Path,
            '#' => Forest,
            '^' | 'v' | '<' | '>' => Slope(Direction::from(c)),
            _ => panic!("Error: Invalid character '{c}'.")
        }
    }
}

#[derive(Eq)]
struct Node {
    position: (usize, usize),
    path_len: usize
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.path_len == other.path_len
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.path_len.cmp(&self.path_len)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    } 
}

impl Node {
    fn new(position: (usize, usize), path_len: usize) -> Self {
        Self {
            position,
            path_len
        }
    }
}

fn read_map(input: &str) -> Vec<Vec<Terrain>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().map(Terrain::from).collect())
        .collect()
}

fn part1(input: &str) -> usize {
    let map = read_map(input);
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();

    let start_position = (0, 1);
    let end_position = (map.len() - 1, map[0].len() - 2);

    let start_node = Node::new(start_position, 0);
    heap.push(start_node);

    let directions = [Up, Down, Left, Right];

    let mut path_lengths: Vec<usize> = Vec::new();

    while let Some(node) = heap.pop() {
        let mut came_from = node.position;
        let mut position = node.position;
        let mut path_len = node.path_len;

        let mut neighbors: Vec<(usize, usize)> = if node.position == start_position {
            vec![(1, 1)]
        } else {
            directions
                .iter()
                .map(|direction| direction.neighbor(node.position))
                .filter(|&neighbor| 
                    neighbor != came_from &&
                    match &map[neighbor.0][neighbor.1] {
                        Path => true,
                        Forest => false,
                        Slope(direction) => node.position != direction.neighbor(neighbor)
                    }
                )
                .collect()
        };

        // normal path, keep going
        while neighbors.len() < 2 {
            path_len += 1;
            came_from = position;
            position = neighbors[0];

            if position == end_position {
                path_lengths.push(path_len);
                neighbors = Vec::new();
                break;
            }

            neighbors = directions
                .iter()
                .map(|direction| direction.neighbor(position))
                .filter(|&neighbor| 
                    neighbor != came_from &&
                    match &map[neighbor.0][neighbor.1] {
                        Path => true,
                        Forest => false,
                        Slope(direction) => position != direction.neighbor(neighbor),
                    }
                )
                .collect();
        }

        // found a node
        for neighbor in neighbors {
            match &map[neighbor.0][neighbor.1] {
                Slope(direction) => match direction {
                    Up => heap.push(Node::new((position.0 - 2, position.1), path_len + 2)),
                    Down => heap.push(Node::new((position.0 + 2, position.1), path_len + 2)),
                    Left => heap.push(Node::new((position.0, position.1 - 2), path_len + 2)),
                    Right => heap.push(Node::new((position.0, position.1 + 2), path_len + 2))
                },
                _ => panic!("Error: Invalid terrain found."),
            }
        }
    }

    path_lengths.into_iter().max().unwrap()
}

fn find_path(
    graph: &HashMap<(usize, usize), Vec<Node>>,
    end_node: (usize, usize),
    current_node: (usize, usize),
    visited_nodes: &mut HashSet<(usize, usize)>,
    path_len: usize
) -> usize {
    if current_node == end_node {
        return path_len;
    }

    let neighbors: Vec<(usize, usize)> = graph
        .get(&current_node)
        .unwrap()
        .iter()
        .filter(|neighbor| visited_nodes.get(&neighbor.position).is_none())
        .map(|node| node.position)
        .collect();

    if neighbors.is_empty() {
        return 0;
    }

    let mut path_lengths: Vec<usize> = Vec::new();
    for neighbor in neighbors {
        visited_nodes.insert(neighbor);

        let edge_weight = graph
            .get(&current_node)
            .unwrap()
            .iter()
            .find(|node| node.position == neighbor)
            .unwrap()
            .path_len;

        path_lengths.push(find_path(graph, end_node, neighbor, visited_nodes, path_len + edge_weight));

        visited_nodes.remove(&neighbor);
    }
    
    path_lengths.into_iter().max().unwrap()
}

fn part2(input: &str) -> usize {
    let map = read_map(input);
    let mut graph: HashMap<(usize, usize), Vec<Node>> = HashMap::new();

    let start_position = (0, 1);
    let end_position = (map.len() - 1, map[0].len() - 2);
    graph.insert(start_position, Vec::new());

    let directions = [Up, Down, Left, Right];
    
    // construct the graph by following the path
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    let mut to_do: VecDeque<((usize, usize), (usize, usize))> = VecDeque::new();
    to_do.push_back((start_position, start_position));

    while let Some((current_position, mut came_from)) = to_do.pop_front() {

        if visited[current_position.0][current_position.1] {
            continue;
        } else {
            visited[current_position.0][current_position.1] = true;
        }

        let mut position = current_position;
        let previous_node = came_from;
        let mut path_len = (current_position != start_position) as usize;

        let mut neighbors: Vec<(usize, usize)> = if current_position == start_position {
            vec![(1, 1)]
        } else {
            directions
                .iter()
                .map(|direction| direction.neighbor(current_position))
                .filter(|&neighbor| 
                    neighbor != came_from
                    && !visited[neighbor.0][neighbor.1]
                    && map[neighbor.0][neighbor.1] != Forest
                )
                .collect()
        };

        // normal path, keep going
        while neighbors.len() < 2 {
            path_len += 1;
            visited[position.0][position.1] = true;
            came_from = position;
            position = neighbors[0];

            // found the end node
            if position == end_position {
                neighbors = Vec::new();

                if let Some(end_node) = graph.get_mut(&position) {
                    end_node.push(Node::new(current_position, path_len));
                } else {
                    graph.insert(end_position, vec![Node::new(current_position, path_len)]);
                }

                break;
            }

            neighbors = directions
                .iter()
                .map(|direction| direction.neighbor(position))
                .filter(|&neighbor| 
                    neighbor != came_from
                    && map[neighbor.0][neighbor.1] != Forest
                )
                .collect();
        }

        let previous_node_adjacency = graph.get_mut(&previous_node).unwrap();

        // found the end node
        if position == end_position {
            previous_node_adjacency.push(Node::new(end_position, path_len));

        // found a junction
        } else {
            previous_node_adjacency.push(Node::new(position, path_len));

            if let Some(node) = graph.get_mut(&position) {
                node.push(Node::new(previous_node, path_len));
            } else {
                graph.insert(position, vec![Node::new(previous_node, path_len)]);
            }
            
            for &neighbor in neighbors.iter() {
                to_do.push_back((neighbor, position));
            }
        }
    }

    // brute force solution
    find_path(&graph, end_position, start_position, &mut HashSet::new(), 0)
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
        let input = r#"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#"#;
        assert_eq!(part1(input), 94);
    }

    #[test]
    fn part_2() {
        let input = r#"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#"#;
        assert_eq!(part2(input), 154);
    }
}