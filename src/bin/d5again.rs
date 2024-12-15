use std::{collections::HashMap, u32};

fn main() {
    //let mut content = include_str!("../data/5.test").split("\n\n");
    let mut content = include_str!("../data/5.in").split("\n\n");

    let rules: Vec<(u32, u32)> = content
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

    let updates: Vec<Vec<u32>> = content
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').filter_map(|num| num.parse().ok()).collect())
        .collect();

    let p1_res = p1(&rules, &updates);
    //let p2_res = p2(&rules, &updates);
    //println!("{p1_res}");
    //println!("{p2_res}");
}

fn update_page_order_rules(rules: &[(u32, u32)], update: &[u32], map: &mut HashMap<u32, Vec<u32>>) {
    for page in update.iter() {
        for rule in rules.iter() {
            if *page == rule.1 && update.contains(&rule.0) {
                map.entry(*page)
                    .and_modify(|v| {
                        if !v.contains(&rule.0) {
                            v.push(rule.0);
                        }
                    })
                    .or_insert_with(|| vec![rule.0]);
            }
        }
    }
}

fn get_update_validity(update: &[u32], page_order_map: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, page) in update.iter().enumerate() {
        let elems_that_shouldnt_follow = page_order_map.get(page);
        let elems_that_follow = &update[i + 1..];
        if has_common_elements(
            elems_that_shouldnt_follow.map(|vec| vec.as_slice()),
            elems_that_follow,
        ) {
            return false;
        }
    }
    true
}

fn has_common_elements(vec: Option<&[u32]>, slice: &[u32]) -> bool {
    match vec {
        Some(vec) => vec.iter().any(|&elem| slice.contains(&elem)),
        None => false,
    }
}

fn p1(rules: &[(u32, u32)], updates: &[Vec<u32>]) -> u32 {
    let mut page_order_rules = HashMap::new();
    let mut res: Vec<u32> = Vec::new();
    for update in updates.iter() {
        update_page_order_rules(rules, &update, &mut page_order_rules);
        if get_update_validity(update, &page_order_rules) {
            res.push(*update.get(update.len() / 2 as usize).unwrap_or(&0_u32));
        }
    }

    res.iter().sum()
}

fn p2(rules: &[(u32, u32)], updates: &[Vec<u32>]) -> u32 {
    let mut page_order_rules = HashMap::new();

    for update in updates.iter() {
        update_page_order_rules(rules, &update, &mut page_order_rules);
    }

    // FIXME: -> filter only wrong (might not be necessary)
    // -> get rules that contain page from current update (here, or here)
    // in degree out degree fn?
    // Kahns algo
    //
    // graph can be represented as set of edges
    //update_page_order_rules(rules, &updates[4], &mut page_order_rules);
    //
    dbg!(&page_order_rules);
    let wrong_order: Vec<_> = updates
        .iter()
        .filter(|upd| !get_update_validity(upd, &page_order_rules))
        .collect();

    dbg!(wrong_order);

    0
}

fn _filter_rules(rules: &[(u32, u32)], update: &[u32]) -> Vec<(u32, u32)> {
    let mut filtered_rules: Vec<(u32, u32)> = Vec::new();

    todo!()
    //for page in update.iter() {}
}
