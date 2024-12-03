fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let reports = include_str!("../data/2.in")
        //let reports = include_str!("../data/2.test")
        .lines()
        .map(read_line_nums)
        .filter(dampen_safety)
        .count();

    dbg!(reports);
    Ok(())
}

fn read_line_nums(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn dampen_safety(report: &Vec<i32>) -> bool {
    //dbg!(report);
    let diffs = report.windows(2).map(|w| w[0] - w[1]).collect::<Vec<_>>();
    //dbg!(&diffs);
    println!("{report:?  }");
    let safe = dbg!(monotone(&diffs) && no_big_jumps(&diffs));

    if safe {
        return true;
    }

    for i in 0..report.len() {
        let dampened_rep = {
            let mut cl = report.clone();
            cl.remove(i); // remove ith to create variant
                          //println!("{:?}", &cl);
            cl
        };

        let diffs = dampened_rep
            .windows(2)
            .map(|w| w[0] - w[1])
            .collect::<Vec<_>>();
        let dampened_safe = dbg!(monotone(&diffs) && no_big_jumps(&diffs));
        if dampened_safe {
            dbg!(report);
            dbg!(&dampened_rep);
            return true;
        }
    }

    false
}

fn monotone(report: &Vec<i32>) -> bool {
    let mut report_iter = report.into_iter();
    let first_sign = report_iter.next().take().unwrap().signum();
    report_iter.all(|s| s.signum() == first_sign)
}

fn no_big_jumps(report: &Vec<i32>) -> bool {
    report.into_iter().all(|x| x.abs() <= 3)
}
