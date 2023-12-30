use std::{error::Error, fs, collections::{HashMap, HashSet}};
use rand::Rng;

fn read_wiring_diagram(input: &str) -> HashMap<String, (usize, Vec<String>)> {
    let mut map: HashMap<String, (usize, HashSet<String>)> = HashMap::new();

    for line in input.lines().map(|line| line.trim()).filter(|line| !line.is_empty()) {
        let mut components: Vec<String> = line.split_whitespace().map(|str| str.to_string()).collect();
        components[0] = components[0].replace(':', "");

        let neighbors = components.iter().skip(1).cloned();
        if let Some(node) = map.get_mut(&components[0]) {
            node.1.extend(neighbors);
        } else {
            map.insert(components[0].clone(), (1, HashSet::from_iter(neighbors)));
        }

        for neighbor in components.iter().skip(1) {
            if let Some(node) = map.get_mut(neighbor) {
                node.1.insert(components[0].clone());
            } else {
                map.insert(neighbor.clone(), (1, HashSet::from_iter(vec![components[0].clone()])));
            }
        }
    }

    let mut ret = HashMap::new();
    for entry in map.into_iter() {
        ret.insert(entry.0, (entry.1.0, entry.1.1.into_iter().collect()));
    }
    ret
}

fn part1(input: &str) -> usize {
    let graph = read_wiring_diagram(input);
    
    // find cut of size 3
    loop {
        let mut cut_graph = graph.clone();
        let mut final_vertex = String::from("");
        
        while cut_graph.len() > 2 {
            
            // choose random edge to contract
            let u = cut_graph.keys().nth(rand::thread_rng().gen_range(0..cut_graph.len())).unwrap().clone();
            let (num_u, n_u) = cut_graph.get(&u).unwrap().clone();

            // choose a random neighbor of u
            let v = n_u.get(rand::thread_rng().gen_range(0..n_u.len())).unwrap().clone();

            // remove all edges to u from u's neighbors
            for string_w in n_u.iter() {
                let w = &mut cut_graph.get_mut(string_w).unwrap().1;
                w.retain(|x| x != &u);
                w.push(v.clone());
            }

            let (num_v, n_v) = cut_graph.get_mut(&v).unwrap();

            // set number of vertices v represents
            *num_v += num_u;

            // contract the neighbors of both vertices and remove self loops
            n_v.extend(n_u);
            n_v.retain(|w| w != &u && w != &v);

            // remove vertex u from the graph
            cut_graph.remove(&u);

            // remember that v exists as a node
            final_vertex = v;
        }

        if cut_graph.get(&final_vertex).unwrap().1.len() == 3 {
            return cut_graph.values().map(|v| v.0).product();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    
    dbg!(part1(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr"#;
        assert_eq!(part1(input), 54);
    }
}