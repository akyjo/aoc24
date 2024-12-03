use itertools::Itertools;
use regex::Regex;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let needle = Regex::new(r"mul\(\d{1,3}\,\d{1,3}\)").unwrap();

    let haystack = include_str!("../data/3.in");
    //let haystack = include_str!("../data/3.test");

    let mul_calls = needle.find_iter(haystack).map(|m| m.as_str()).collect_vec();

    let filtered = mul_calls
        .iter()
        .map(|st| {
            st.chars()
                .filter_map(|x| match x {
                    ',' => Some(' '),
                    '0'..='9' => Some(x),
                    _ => None,
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    let result = filtered
        .iter()
        .map(|st| {
            let mut strnums = st.split_whitespace();
            strnums.next().unwrap().parse::<i32>().unwrap()
                * strnums.next().unwrap().parse::<i32>().unwrap()
        })
        .sum::<i32>();
    dbg!(result);
    Ok(())
}
