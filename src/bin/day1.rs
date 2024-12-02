use itertools::Itertools;
use std::collections::HashMap;
use std::iter::zip;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    part_one();
    part_two();

    Ok(())
}

fn part_one() {
    let foo: (Vec<u32>, Vec<u32>) = include_str!("../data/1.in")
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let left = it.next().unwrap().parse::<u32>().unwrap();
            let right = it.last().unwrap().parse::<u32>().unwrap();
            (left, right)
        })
        .unzip();

    let (mut left, mut right) = foo;
    left.sort();
    right.sort();

    let answer: i32 = zip(left, right)
        .map(|a| (a.0 as i32 - a.1 as i32).abs())
        .sum();
    dbg!(answer);
}

fn part_two() {
    let foo: (Vec<u32>, Vec<u32>) = include_str!("../data/1.in")
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let left = it.next().unwrap().parse::<u32>().unwrap();
            let right = it.last().unwrap().parse::<u32>().unwrap();
            (left, right)
        })
        .unzip();

    let (left, right) = foo;
    let mut left_frequencies = left
        .into_iter()
        .unique()
        .map(|x| (x, 0_u32))
        .collect::<HashMap<u32, u32>>();

    for num in right.into_iter() {
        match left_frequencies.get(&num) {
            Some(_) => {
                left_frequencies.entry(num).and_modify(|count| *count += 1);
            }
            None => {
                ();
            }
        }
    }

    let answer = left_frequencies
        .into_iter()
        .filter(|entry| entry.1 > 0)
        .map(|entry| entry.0 * entry.1)
        .sum::<u32>();
    dbg!(answer);
}
