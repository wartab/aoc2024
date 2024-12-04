use crate::day4::INPUT;

pub fn part1() {
    let grid = INPUT.split("\n").map(|line|
        line.trim().chars().collect()
    ).collect::<Vec<Vec<char>>>();

    let height = grid.len();
    let width = grid[0].len();

    let mut count = 0usize;

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'X' {
                count += find_horizontal(&grid[i], j);
                count += find_vertical(&grid, i, j);
                count += find_diagonal(&grid, i, j);
            }
        }
    }

    println!("Count: {count}");
}

fn find_diagonal(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    // Bottom right
    let mut count = match (
        grid.get(i + 1).and_then(|row| row.get(j + 1)),
        grid.get(i + 2).and_then(|row| row.get(j + 2)),
        grid.get(i + 3).and_then(|row| row.get(j + 3)),
    ) {
        (Some('M'), Some('A'), Some('S')) => 1,
        _ => 0,
    };

    // Bottom left

    if j >= 3 {
        count += match (
            grid.get(i + 1).and_then(|row| Some(row[j - 1])),
            grid.get(i + 2).and_then(|row| Some(row[j - 2])),
            grid.get(i + 3).and_then(|row| Some(row[j - 3])),
        ) {
            (Some('M'), Some('A'), Some('S')) => 1,
            _ => 0,
        };
    }

    // Top left
    if j >= 3 && i >= 3 {
        count += match (
            grid[i - 1][j - 1],
            grid[i - 2][j - 2],
            grid[i - 3][j - 3],
        ) {
            ('M', 'A', 'S') => 1,
            _ => 0,
        };
    }

    // Top right
    if i >= 3 {
        count += match (
            grid[i - 1].get(j + 1),
            grid[i - 2].get(j + 2),
            grid[i - 3].get(j + 3),
        ) {
            (Some('M'), Some('A'), Some('S')) => 1,
            _ => 0,
        };
    }

    count
}

fn find_vertical(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    let mut count = match (
        grid.get(i + 1).and_then(|row| Some(row[j])),
        grid.get(i + 2).and_then(|row| Some(row[j])),
        grid.get(i + 3).and_then(|row| Some(row[j])),
    ) {
        (Some('M'), Some('A'), Some('S')) => 1,
        _ => 0,
    };

    if i >= 3 {
        count += match (
            grid.get(i - 1).and_then(|row| Some(row[j])),
            grid.get(i - 2).and_then(|row| Some(row[j])),
            grid.get(i - 3).and_then(|row| Some(row[j])),
        ) {
            (Some('M'), Some('A'), Some('S')) => 1,
            _ => 0,
        };
    }

    count
}

fn find_horizontal(line: &[char], offset: usize) -> usize {
    let mut count = match (line.get(offset + 1), line.get(offset + 2), line.get(offset + 3)) {
        (Some('M'), Some('A'), Some('S')) => 1,
        _ => 0,
    };

    if offset >= 3 {
        count += match (line.get(offset - 1), line.get(offset - 2), line.get(offset - 3)) {
            (Some('M'), Some('A'), Some('S')) => 1,
            _ => 0,
        };
    }

    count
}