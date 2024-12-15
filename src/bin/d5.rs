fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    //let mut input = include_str!("../data/5.test").split("\n\n");
    let mut input = include_str!("../data/5.in").split("\n\n");

    let rules: Vec<(u32, u32)> = input
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('|');
            let a = parts.next()?.parse().ok()?;
            let b = parts.next()?.parse().ok()?;
            Some((a, b))
        })
        .collect();

    let updates: Vec<Vec<u32>> = input
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').filter_map(|num| num.parse().ok()).collect())
        .collect();

    let (correct, incorrect): (Vec<_>, Vec<_>) = updates
        .iter()
        .partition(|update| update_is_path(update, &rules));

    let p1: u32 = correct
        .iter()
        .cloned()
        .map(|updt| updt.get((updt.len() / 2) as usize).unwrap())
        .sum();

    println!("{p1}");

    let mut p2: u32 = 0;
    for update in incorrect.iter() {
        //println!("update is: {update:?}");
        //let sorted = kahns_algo(&subgraph_from_update(&update, &rules))
        let sorted = dfs_topological_sort(&subgraph_from_update(&update, &rules))
            .ok()
            .unwrap();
        //println!("sorted update is: {sorted:?}");
        p2 += sorted[(sorted.len() / 2) as usize];
    }

    println!("{p2}");

    Ok(())
}

fn update_is_path(update: &[u32], rules: &[(u32, u32)]) -> bool {
    let potential_path: Vec<(u32, u32)> = update
        .windows(2)
        .map(|a| (a[0].to_owned(), a[1].to_owned()))
        .collect();

    for edge in potential_path.iter() {
        if rules.contains(edge) {
            continue;
        } else {
            return false;
        }
    }
    true
}

fn subgraph_from_update(update: &[u32], rules: &[(u32, u32)]) -> Vec<(u32, u32)> {
    rules
        .iter()
        .filter(|rule| update.contains(&rule.0) && update.contains(&rule.1))
        .copied()
        .collect()
}

// IDK WHATS WRONG AND MY ALGO GAME IS TOO WEAK
//fn kahns_algo(edges: &Vec<(u32, u32)>) -> Result<Vec<u32>, &'static str> {
//    use std::collections::{HashMap, VecDeque};
//
//    let mut in_degree = HashMap::new();
//
//    for &(from, to) in edges {
//        *in_degree.entry(to).or_insert(0) += 1;
//        in_degree.entry(from).or_insert(0);
//    }
//
//    let mut no_incoming: VecDeque<u32> = in_degree
//        .iter()
//        .filter(|&(_, &count)| count == 0)
//        .map(|(&node, _)| node)
//        .collect();
//
//    let mut sorted = Vec::new();
//    let mut edges = edges.clone();
//
//    while let Some(n) = no_incoming.pop_front() {
//        sorted.push(n);
//
//        let mut remaining_edges = Vec::new();
//
//        for (from, to) in edges {
//            if from == n {
//                if let Some(degree) = in_degree.get_mut(&to) {
//                    *degree -= 1;
//                    if *degree == 0 {
//                        no_incoming.push_back(to);
//                    }
//                } else {
//                    remaining_edges.push((from, to));
//                }
//            }
//        }
//        edges = remaining_edges;
//    }
//
//    if edges.is_empty() {
//        Ok(sorted)
//    } else {
//        Err("Graph has a cycle")
//    }
//}

fn dfs_topological_sort(edges: &Vec<(u32, u32)>) -> Result<Vec<u32>, &'static str> {
    use std::collections::HashSet;

    fn neighbors(node: u32, edges: &Vec<(u32, u32)>) -> Vec<u32> {
        edges
            .iter()
            .filter(|&&(from, _)| from == node)
            .map(|&(_, to)| to)
            .collect()
    }

    fn visit(
        node: u32,
        edges: &Vec<(u32, u32)>,
        visiting: &mut HashSet<u32>,
        visited: &mut HashSet<u32>,
        sorted: &mut Vec<u32>,
    ) -> Result<(), &'static str> {
        if visited.contains(&node) {
            return Ok(());
        }
        if visiting.contains(&node) {
            return Err("Graph has at least one cycle");
        }

        visiting.insert(node);

        for neighbor in neighbors(node, edges) {
            visit(neighbor, edges, visiting, visited, sorted)?;
        }

        visiting.remove(&node);
        visited.insert(node);
        sorted.push(node);
        Ok(())
    }

    let nodes: HashSet<u32> = edges
        .iter()
        .flat_map(|&(from, to)| vec![from, to])
        .collect();

    let mut sorted = Vec::new();
    let mut visited = HashSet::new();
    let mut visiting = HashSet::new();

    for &node in &nodes {
        if !visited.contains(&node) {
            visit(node, edges, &mut visiting, &mut visited, &mut sorted)?;
        }
    }

    sorted.reverse();
    Ok(sorted)
}
