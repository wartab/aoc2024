use crate::day4::INPUT;

pub fn part2() {
    let grid = INPUT.split("\n").map(|line|
        line.trim().chars().collect()
    ).collect::<Vec<Vec<char>>>();

    let height = grid.len();
    let width = grid[0].len();

    let mut count = 0usize;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if grid[i][j] == 'A' &&
                check_mas(grid[i - 1][j - 1], grid[i + 1][j + 1]) &&
                check_mas(grid[i - 1][j + 1], grid[i + 1][j - 1]) {
                count += 1;
            }
        }
    }

    println!("Count: {count}");
}

fn check_mas(a: char, b: char) -> bool {
    (a == 'M' && b == 'S') || (a == 'S' && b == 'M')
}
