use std::collections::HashSet;
use std::hash::Hash;
use std::io::BufRead;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "X:\\Personal\\AdventOfCode\\DATASET\\six\\pm.txt";
    let filepath = std::path::Path::new(filename);

    let file = std::fs::File::open(&filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut original_santa = Santa {
        col_index: 0,
        row_index: 0,
        direction: Direction::Up,
        quitted: false,
    };

    let mut blocks_by_row: Vec<Vec<usize>> = Vec::new();
    let mut blocks_by_column: Vec<Vec<usize>> = Vec::new();

    for (row_idx, row) in reader.lines().enumerate() {
        let row = row?;
        if row_idx == 0 {
            for _ in 0.. row.len() {
                blocks_by_column.push(Vec::new());
            }
        }

        let mut row_blocks:Vec<usize> = Vec::new();
        for block_idx in row.chars().enumerate().filter(|(_, _char)| *_char == '#').map(|(idx, _)| idx) {
            row_blocks.push(block_idx);
            blocks_by_column[block_idx].push(row_idx);
        }
        blocks_by_row.push(row_blocks);

        if let Some(santa_pos) = row.find('^') {
            original_santa = Santa {
                row_index: row_idx,
                col_index: santa_pos,
                direction: Direction::Up,
                quitted: false,
            };
        }
    }

    let total_rows = blocks_by_row.len();
    let total_cols = blocks_by_column.len();

    // fill matrix
    println!("Creating a matrix of {total_rows}rows x {total_cols}columns");
    let mut original_map: Vec<Vec<u8>> = vec![vec![0; total_cols]; total_rows]; // 0=unvisited, 1=visited, 2=block
    for (br_idx, br) in blocks_by_row.iter().enumerate() {
        for bc in br {
            original_map[br_idx][*bc] = 2; // block
        }
    }

    let mut map = original_map.clone();
    let mut santa = original_santa.clone();

    // count visited positions
    map[santa.row_index][santa.col_index] = 1;
    let mut visited = 1;

    while !santa.quitted {
        let (new_visited, _) = move_santa(&mut santa, &mut map, total_rows, total_cols);
        visited += new_visited;
    }

    // check loops adding one block
    let mut loops:u32 = 0;
    for (row_idx, col) in map.iter().enumerate() {
        for (col_idx, value) in col.iter().enumerate() {
            if *value == 1 && !(row_idx == original_santa.row_index && col_idx == original_santa.col_index) {
                // test loops
                let mut test_map = original_map.clone();
                let mut test_santa = original_santa.clone();
                test_map[row_idx][col_idx] = 2; // contains block
                if map_contains_loop(&mut test_map, &mut test_santa, total_rows, total_cols) {
                    loops += 1;
                }
            }
        }
    }
    
    println!("Visited: {visited}, possible loops {loops} in {:.2?}", init_ts.elapsed());
    Ok(())
}

fn map_contains_loop(map: &mut Vec<Vec<u8>>, santa: &mut Santa, total_rows: usize, total_cols: usize) -> bool {
    let mut blocks: HashSet<Block> = HashSet::new();
    while !santa.quitted {
        let (_, block) = move_santa(santa, map, total_rows, total_cols);

        if let Some(block) = block {
            if blocks.insert(block) == false { // blocked before => loop!
                return true;
            }
        }
    }

    return false;
}

// 0=unvisited, 1=visited, 2=block
fn move_santa(santa: &mut Santa, map: &mut Vec<Vec<u8>>, total_rows: usize, total_cols: usize) -> (i32, Option<Block>) {
    let mut visited: i32 = 0;

    loop {
        if santa.direction == Direction::Up && santa.row_index == 0 {
            santa.quitted = true;
        } else if santa.direction == Direction::Down && santa.row_index == total_rows - 1 {
            santa.quitted = true;
        } else if santa.direction == Direction::Right && santa.col_index == total_cols - 1 {
            santa.quitted = true;
        } else if santa.direction == Direction::Left && santa.col_index == 0 {
            santa.quitted = true;
        }

        if santa.quitted {
            return (visited, None);
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
            santa.update_position(next_row, next_col);
        } else if next_block == 1 {
            santa.update_position(next_row, next_col);
        } else if next_block == 2 {
            // block
            let block = Block {
                row_index: santa.row_index,
                col_index: santa.col_index,
                direction: santa.direction
            };
            santa.direction = santa.direction.get_next_direction();

            return (visited, Some(block));
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn get_next_direction(&self) -> Direction {
        return match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        };
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Block {
    row_index: usize,
    col_index: usize,
    direction: Direction
}

#[derive(Clone, Copy)]
struct Santa {
    row_index: usize,
    col_index: usize,
    direction: Direction,
    quitted: bool,
}

impl Santa {
    fn update_position(&mut self, row_index: usize, col_index: usize) {
        self.row_index = row_index;
        self.col_index = col_index;
    }
}