use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("inputs/input.txt")?;
//     let reader = io::BufReader::new(file);
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect::<Vec<Vec<char>>>();

    // count XMAS (i.e. cross-diagonal MAS, on a 3x3 grid, centered at A, with M, S at opposite corners)
    let mut count = 0;

    let rows = lines.len();
    let cols = if rows > 0 { lines[0].len() } else { 0 };

    for i in 0..rows {
        for j in 0..cols {
            if is_match(&lines, i as isize, j as isize) {
                count += 1;
                // println!("Found cross MAS at ({}, {})", i, j);
            }
        }
    }

    println!("{}", count);
    Ok(())
}

fn is_match(
    grid: &Vec<Vec<char>>,
    start_row: isize,
    start_col: isize,
) -> bool {
    let rows = grid.len() as isize;
    let cols = if rows > 0 { grid[0].len() as isize } else { 0 };

    // check if current cell is A:
    if start_row < 0 || start_row >= rows || start_col < 0 || start_col >= cols || grid[start_row as usize][start_col as usize] != 'A' {
        return false;
    }

    let mut count_m1 = 0;
    let mut count_s1 = 0;
    let mut count_m2 = 0;
    let mut count_s2 = 0;

    let dirs1: [(i32, i32); 2] = [
        (-1, -1),
        (1, 1),
    ];

    let dirs2: [(i32, i32); 2] = [
        (-1, 1),
        (1, -1),
    ];
    
    for dir in &dirs1 {
        let r = start_row + dir.0 as isize;
        let c = start_col + dir.1 as isize;
        if r >= 0 && r < rows && c >= 0 && c < cols {
            if grid[r as usize][c as usize] == 'M' {
                count_m1 += 1;
            } else if grid[r as usize][c as usize] == 'S' {
                count_s1 += 1;
            }
        } else {
            return false;
        }
    }
    if !(count_m1 + count_s1 == 2 && count_m1 == count_s1) {
        return false;
    }
    for dir in &dirs2 {
        let r = start_row + dir.0 as isize;
        let c = start_col + dir.1 as isize;
        if r >= 0 && r < rows && c >= 0 && c < cols {
            if grid[r as usize][c as usize] == 'M' {
                count_m2 += 1;
            } else if grid[r as usize][c as usize] == 'S' {
                count_s2 += 1;
            }
        } else {
            return false;
        }
    }

count_m2 + count_s2 == 2 && count_m2 == count_s2
}