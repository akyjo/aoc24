use regex::Regex;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mul_calls = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let mul_dos_donts = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)")?;

    let haystack = include_str!("../data/3.in");

    let part_one_result = p1(&mul_calls, &haystack);
    let part_two_result = p2(&mul_dos_donts, &haystack);

    println!("Part One: {}", part_one_result);
    println!("Part Two: {}", part_two_result);

    Ok(())
}

fn p1(mul_calls: &Regex, input: &str) -> i32 {
    mul_calls
        .captures_iter(input)
        .map(|caps| {
            let a: i32 = caps[1].parse().unwrap();
            let b: i32 = caps[2].parse().unwrap();
            a * b
        })
        .sum()
}

fn p2(mul_dos_donts: &Regex, input: &str) -> i32 {
    let mut mul_legal = true;
    let mut sum = 0;

    for caps in mul_dos_donts.captures_iter(input) {
        match &caps[0] {
            "do()" => mul_legal = true,
            "don't()" => mul_legal = false,
            other if mul_legal && other.starts_with("mul(") => {
                let a: i32 = caps[1].parse().unwrap();
                let b: i32 = caps[2].parse().unwrap();
                sum += a * b;
            }
            _ => (),
        }
    }

    sum
}
