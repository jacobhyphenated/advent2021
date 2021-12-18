/*
Day 12: Passage Pathing

A series of caves are connected from tunnels. Caves with lower case names are small caves.
Start at the cave labeled "start" and end at the cave labeled "end"

Part 1: Map all possible paths through the caves. No path can pass through the same small cave twice.

Part 2: Map all possible paths, but this time, any one sigle small cave can be visited twice.
*/

use std::collections::HashMap;
use std::fs;

// The struct mostly exists because I wanted to build a graph with edges.
// But I had to abandon that approach due to being bad at Rust.
// https://github.com/nrc/r4cppp/blob/master/graphs/README.md
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Cave {
    name: String,
    is_large: bool,
}

impl Cave {
    fn new(name: String) -> Cave {
        let is_large = name == name.to_ascii_uppercase();
        Cave { name, is_large }
    }
}

// Part 1: Most logic is combined with part 2
pub fn count_total_paths(graph: &HashMap<Cave, Vec<Cave>>) -> usize {
    let start = graph.keys().find(|cave| cave.name == "start").unwrap();
    return recurse_paths(&start, &vec![], &graph, false).unwrap().len();
}

// Part 2
pub fn count_paths_visit_twice(graph: &HashMap<Cave, Vec<Cave>>) -> usize {
    let start = graph.keys().find(|cave| cave.name == "start").unwrap();
    return recurse_paths(&start, &vec![], &graph, true).unwrap().len();
}

/**
 * Recursive method that finds the next step in a path.
 * root - the current cave we are in
 * path - list of caves we have visited to get to this point
 * graph - representation of the cave system
 * double_pass - flag for part 1 vs part 2 rules
 * 
 * First, look to see if we are in an invalid path state, if so, return None
 * If we are at the "end" return this exact path
 * Otherwise, create a series of potential paths by calling recurse_paths on all adjacent caves
 * 
 * Bonus: I did lifetimes! A small consolation for failing at a graph structure
 */
fn recurse_paths<'a>(root: &'a Cave, path: &Vec<&'a Cave>, graph: &'a HashMap<Cave, Vec<Cave>>, double_pass: bool) -> Option<Vec<Vec<&'a Cave>>> {
    // Cannot traverse a small cave twice
    if !double_pass && !root.is_large && path.contains(&root) {
        return None;
    }
    // allow traversing a single small cave twice (but not "start")
    else if double_pass {
        if root.name == "start" && path.len() > 0 {
            return None;
        }
        let small_count: HashMap<&Cave, i32> = path.iter()
            .filter(|c| !c.is_large)
            .fold(HashMap::new(), |mut map, cave| {
                *map.entry(cave).or_insert(0) += 1;
                map
            });
        if small_count.contains_key(root) && small_count.values().any(|&count| count > 1) {
            return None;
        }
    }

    // clone path - we make a new path vector for each choice of next cave
    let mut current_path = path.clone();
    current_path.push(root);
    if root.name == "end" {
        return Some(vec![current_path])
    }

    // filter_amp removes Nones - those paths are dead ends
    // flat map to reduce back to a list of "paths", rather than a list of list of paths.
    Some(graph.get(root).unwrap().iter()
        .filter_map(|adjacent| recurse_paths(adjacent, &current_path, &graph, double_pass))
        .flat_map(|p| p)
        .collect())

}

pub fn read_paths() -> HashMap<Cave, Vec<Cave>> {
    let input = fs::read_to_string("src/day12/paths.txt").expect("missing paths.txt");
    parse_input(&input)
}

fn parse_input(input: &str) -> HashMap<Cave, Vec<Cave>> {
    let mut graph: HashMap<Cave, Vec<Cave>> = HashMap::new();

    // map together caves - but unable to map to references of caves (instead, .clone() a bunch)
    // this is definitely the wrong way to do this, the right way probably involves Rc<RefCell<Cave>> or something
    // Graphs are an especially hard problem in rust.
    for line in input.lines() {
        let nodes: Vec<_> = line.trim().split("-").collect();
        let c1 = Cave::new(nodes[0].to_string());
        let c2 = Cave::new(nodes[1].to_string());

        let c1_map = graph.entry(c1.clone()).or_insert(vec![]);
        c1_map.push(c2.clone());
        let c2_map = graph.entry(c2).or_insert(vec![]);
        c2_map.push(c1);
    }

    return graph;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_paths_simple() {
        let input = "start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end";
        let graph = parse_input(input);
        assert_eq!(10, count_total_paths(&graph));
        assert_eq!(36, count_paths_visit_twice(&graph));
    }

    #[test]
    fn test_all_paths_advanced() {
        let input = "fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW";
        let graph = parse_input(input);
        assert_eq!(226, count_total_paths(&graph));
        assert_eq!(3509, count_paths_visit_twice(&graph));
    }

}