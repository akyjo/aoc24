fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    //let reports = include_str!("../data/2.in")
    let reports = include_str!("../data/2.test")
        .lines()
        .map(read_line_nums)
        .map(|nums| nums.windows(2).map(|w| (w[0] - w[1])).collect::<Vec<_>>())
        .filter(monotone)
        .filter(no_big_jumps)
        .count();

    //let answer = reports.into_iter().map();

    dbg!(reports);
    Ok(())
}

fn monotone(report: &Vec<i32>) -> bool {
    dbg!(report);
    let mut report_iter = report.into_iter();
    let first_sign = report_iter.next().take().unwrap().signum();
    report_iter.all(|s| s.signum() == first_sign)
}

fn no_big_jumps(report: &Vec<i32>) -> bool {
    report.into_iter().all(|x| x.abs() <= 3)
}

fn read_line_nums(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}
