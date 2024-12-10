use std::collections::btree_map::Range;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::BufRead;
use std::ops::Index;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "D:\\Personal\\AdventOfCode\\DATASET\\six\\input.txt";
    let filepath = std::path::Path::new(filename);

    let file = std::fs::File::open(&filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut santa = Santa {
        col_index: 0,
        row_index: 0,
        direction: Direction::Up,
        quitted: false,
    };

    let mut blocks_by_row: Vec<Vec<usize>> = Vec::new();
    for (row_idx, row) in reader.lines().enumerate() {
        let row = row?;
        blocks_by_row.push(
            row.chars()
                .enumerate()
                .filter(|(_, char)| *char == '#')
                .map(|(idx, _)| idx)
                .collect(),
        );

        if let Some(santa_pos) = row.find('^') {
            santa = Santa {
                row_index: row_idx,
                col_index: santa_pos,
                direction: Direction::Up,
                quitted: false,
            };
        }
    }

    let total_rows = blocks_by_row.len();
    let total_columns = *blocks_by_row
        .iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.last().unwrap())
        .max()
        .unwrap() + 1;

    // fill matrix
    println!("Creating a matrix of {total_rows}rows x {total_columns}columns");
    let mut map: Vec<Vec<u8>> = vec![vec![0; total_columns]; total_rows]; // 0=unvisited, 1=visited, 2=block
    for (br_idx, br) in blocks_by_row.iter().enumerate() {
        for bc in br {
            map[br_idx][*bc] = 2; // block
        }
    }

    map[santa.row_index][santa.col_index] = 1;
    let mut visited = 1;

    while !santa.quitted {
        visited += move_santa(&mut santa, &mut map, total_rows, total_columns);
    }

    // let mut santa_row: usize;
    // let mut santa_col: usize;
    // let mut blocks_by_row: Vec<Vec<usize>> = Vec::new();
    // let mut blocks_by_column: Vec<Vec<usize>> = Vec::new();

    // for (row_idx, row) in reader.lines().enumerate() {
    //     let row = row?;
    //     if row_idx == 0 {
    //         for col in 0.. row.len() {
    //             blocks_by_column.push(Vec::new());
    //         }
    //     }

    //     let mut row_blocks:Vec<usize> = Vec::new();

    //     for block_idx in row.chars().enumerate().filter(|(idx, char)| *char == '#').map(|(idx, char)| idx) {
    //         row_blocks.push(block_idx);
    //         blocks_by_column.get(block_idx).unwrap().push(row_idx);
    //     }
    //     blocks_by_row.push(row_blocks);

    //     if let Some(santa_pos) = row.find('^') {
    //         santa_row = row_idx;
    //         santa_col = santa_pos;
    //     }
    // }

    // let total_rows = blocks_by_row.len();
    // let total_columns = blocks_by_column.len();
    // let direction =  Direction::Up;
    // // iterate up-down-left-right and store ranges

    // let mut is_quitted = false;
    // while !is_quitted {
    //     santa_row, santa_col, horizonal_range

    // }

    println!("Visited: {visited} in {:.2?}", init_ts.elapsed());
    Ok(())
}

// 0=unvisited, 1=visited, 2=block
fn move_santa(santa: &mut Santa, map: &mut Vec<Vec<u8>>, total_rows: usize, total_cols: usize) -> i32 {
    let mut visited: i32 = 0;

    loop {
        if santa.direction == Direction::Up && santa.row_index == 0 {
            santa.quitted = true;
            return visited;
        } else if santa.direction == Direction::Down && santa.row_index == total_rows - 1 {
            santa.quitted = true;
            return visited;
        } else if santa.direction == Direction::Right && santa.col_index == total_cols - 1 {
            santa.quitted = true;
            return visited;
        } else if santa.direction == Direction::Left && santa.row_index == 0 {
            santa.quitted = true;
            return visited;
        }

        let next_row = match santa.direction {
            Direction::Up => santa.row_index - 1,
            Direction::Down => santa.row_index + 1,
            Direction::Left => santa.row_index,
            Direction::Right => santa.row_index
        };

        let next_col = match santa.direction {
            Direction::Left => santa.col_index - 1,
            Direction::Right => santa.col_index + 1,
            Direction::Up => santa.col_index,
            Direction::Down => santa.col_index
        };

        let next_block = map[next_row][next_col];
        if next_block == 0 {
            visited += 1;
            map[next_row][next_col] = 1;
            santa.row_index = next_row;
            santa.col_index = next_col;
        } else if next_block == 1 {
            santa.row_index = next_row;
            santa.col_index = next_col;
        } else if next_block == 2 {
            // block
            santa.direction = match santa.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up
            };
            return visited;
        }
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Santa {
    row_index: usize,
    col_index: usize,
    direction: Direction,
    quitted: bool,
}
