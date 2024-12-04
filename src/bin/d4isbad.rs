struct Puzzle {
    grid: Vec<Vec<char>>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("../data/4.in")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut puzzle = Puzzle { grid: input };

    let part_one_result = p1(&mut puzzle);
    //let part_two_result = p2();
    //
    println!("Part One: {}", part_one_result);
    //println!("Part Two: {}", part_two_result);

    Ok(())
}

//(-1, -1), (-1, 0), (-1, 1),
//( 0, -1),    X   , ( 0, 1),
//( 1, -1), ( 1, 0), ( 1, 1),
fn find_xmas(grid: &Vec<Vec<char>>, row: isize, col: isize, directions: &[(isize, isize)]) -> u32 {
    let word = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    for &(dx, dy) in directions {
        let mut r = row;
        let mut c = col;
        let mut matched = true;

        for &ch in &word {
            if r < 0 || c < 0 || r >= grid.len() as isize || c >= grid[0].len() as isize {
                matched = false;
                break;
            }
            if grid[r as usize][c as usize] != ch {
                matched = false;
                break;
            }
            r += dx;
            c += dy;
        }

        if matched {
            count += 1;
        }
    }

    count
}

fn p1(puzzle: &Puzzle) -> u32 {
    let directions = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;

    for i in 0..puzzle.grid.len() {
        for j in 0..puzzle.grid[i].len() {
            if puzzle.grid[i][j] == 'X' {
                count += find_xmas(&puzzle.grid, i as isize, j as isize, &directions);
            }
        }
    }

    count
}
