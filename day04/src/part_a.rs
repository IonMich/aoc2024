use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("inputs/input.txt")?;
//     let reader = io::BufReader::new(file);
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect::<Vec<Vec<char>>>();

    let word = "XMAS".chars().collect::<Vec<char>>();
    let mut count = 0;

    let directions = [
        (-1, -1), // Up-Left
        (-1, 0),  // Up
        (-1, 1),  // Up-Right
        (0, -1),  // Left
        (0, 1),   // Right
        (1, -1),  // Down-Left
        (1, 0),   // Down
        (1, 1),   // Down-Right
    ];

    let rows = lines.len();
    let cols = if rows > 0 { lines[0].len() } else { 0 };

    for i in 0..rows {
        for j in 0..cols {
            for dir in &directions {
                if is_match(&lines, &word, i as isize, j as isize, dir.0, dir.1) {
                    count += 1;
                    // println!("Found word at ({}, {}) in direction {:?}", i, j, dir);
                }
            }
        }
    }

    println!("{}", count);
    Ok(())
}

fn is_match(
    grid: &Vec<Vec<char>>,
    word: &Vec<char>,
    start_row: isize,
    start_col: isize,
    dir_row: isize,
    dir_col: isize,
) -> bool {
    let rows = grid.len() as isize;
    let cols = if rows > 0 { grid[0].len() as isize } else { 0 };

    let mut row = start_row;
    let mut col = start_col;

    for &ch in word {
        if row < 0 || row >= rows || col < 0 || col >= cols {
            return false;
        }
        if grid[row as usize][col as usize] != ch {
            return false;
        }
        row += dir_row;
        col += dir_col;
    }
    true
}

