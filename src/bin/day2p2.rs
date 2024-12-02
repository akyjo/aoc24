use std::collections::HashMap;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    //let reports = include_str!("../data/2.in")
    let reports = include_str!("../data/2.test")
        .lines()
        .map(read_line_nums)
        .map(|nums| nums.windows(2).map(|w| (w[0] - w[1])).collect::<Vec<_>>())
        .filter(dampen_monotone)
        .filter(allow_one_jump)
        .count();

    //let answer = reports.into_iter().map();

    dbg!(reports);
    Ok(())
}

fn dampen_monotone(report: &Vec<i32>) -> bool {
    dbg!(report);
    let mut report_iter = report.iter();
    let first_sign = report_iter.next().take().unwrap().signum();
    match report_iter.all(|s| s.signum() == first_sign) {
        true => true,
        false => try_damper(report),
    }
}

///
///    1        5       1       1
///  (2 - 1) (7 - 2) (8 - 7) (9 - 8)
/// 1       2       7       8       9
///
///     2      -1       2       1
///  (3 - 1) (2 - 3) (4 - 2) (5 - 4)
/// 1       3       2       4       5
///
fn try_damper(report: &Vec<i32>) -> bool {
    let valid_dampened = report
        .iter()
        .map(|s| s.signum())
        .fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|freq| *freq += 1).or_insert(1);
            map
        })
        .values()
        .filter(|x| **x <= 1)
        .sum::<i32>();

    valid_dampened <= 1
}

fn allow_one_jump(report: &Vec<i32>) -> bool {
    match report.iter().all(|x| x.abs() <= 3) {
        true => true,
        false => jump_damper(report),
    }
}

fn jump_damper(report: &Vec<i32>) -> bool {
    let jump_count = report.iter().filter(|x| **x >= 4).collect::<Vec<_>>().len();
    jump_count <= 1_usize
}

fn read_line_nums(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}
