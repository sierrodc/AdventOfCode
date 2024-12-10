use std::cmp;
use std::io::BufRead;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "X:\\Personal\\AdventOfCode\\DATASET\\six\\test.txt";
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
            santa = Santa {
                row_index: row_idx,
                col_index: santa_pos,
                direction: Direction::Up,
                quitted: false,
            };
        }
    }

    let total_rows = blocks_by_row.len();
    let total_columns = blocks_by_column.len();

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
    let mut straight_paths: Vec<StraightPath> = Vec::new();

    let mut loops = 0;
    while !santa.quitted {
        let (new_visited, new_straight_path) = move_santa(&mut santa, &mut map, total_rows, total_columns);

        println!("{new_straight_path:?}");
        // check if there's an intersection with previous direction
        loops += straight_paths.iter()
            .filter(|s| s.intersect(&new_straight_path))
            .count();

        straight_paths.push(new_straight_path);
        visited += new_visited;
    }

    println!("Visited: {visited}, possible loops {loops} in {:.2?}", init_ts.elapsed());
    Ok(())
}

// 0=unvisited, 1=visited, 2=block
fn move_santa(santa: &mut Santa, map: &mut Vec<Vec<u8>>, total_rows: usize, total_cols: usize) -> (i32, StraightPath) {
    let mut visited: i32 = 0;

    let mut sp = StraightPath {
        direction: santa.direction,
        from_col_index: santa.col_index,
        from_row_index: santa.row_index,
        to_col_index: 0,
        to_row_index: 0
    };

    loop {
        if santa.direction == Direction::Up && santa.row_index == 0 {
            santa.quitted = true;
        } else if santa.direction == Direction::Down && santa.row_index == total_rows - 1 {
            santa.quitted = true;
        } else if santa.direction == Direction::Right && santa.col_index == total_cols - 1 {
            santa.quitted = true;
        } else if santa.direction == Direction::Left && santa.row_index == 0 {
            santa.quitted = true;
        }

        if santa.quitted {
            sp.to_col_index = santa.col_index;
            sp.to_row_index = santa.row_index;
            return (visited, sp);
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
            santa.direction = santa.direction.get_next_direction();
            sp.to_col_index = santa.col_index;
            sp.to_row_index = santa.row_index;

            return (visited, sp);
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

    fn is_horizontal(&self) -> bool {
        return match self {
            Direction::Up => false,
            Direction::Right => true,
            Direction::Down => false,
            Direction::Left => true
        };
    }
}

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
#[derive(Debug)]
struct StraightPath {
    from_row_index: usize,
    to_row_index: usize,
    from_col_index: usize,
    to_col_index: usize,
    direction: Direction
}

impl StraightPath {
    fn intersect(&self, next_path: &StraightPath) -> bool {
        if self.direction != next_path.direction.get_next_direction() {
            return false;
        }

        println!("{self:?} vs {next_path:?}");

        let mut intersection = false;
        if self.direction.is_horizontal() {
            let self_row = self.from_row_index;
            let self_from_col = cmp::min(self.from_col_index, self.to_col_index);
            let self_to_col = cmp::max(self.from_col_index, self.to_col_index);
            
            let next_col = next_path.from_col_index;
            let next_from_row = cmp::min(next_path.from_row_index, next_path.to_row_index);
            let next_to_row = cmp::max(next_path.from_row_index, next_path.to_row_index);

            intersection = self_from_col <= next_col && self_to_col >= next_col && 
                        next_from_row <= self_row && next_to_row >= self_row;
        } else {
            let self_col = self.from_col_index;
            let self_from_row = cmp::min(self.from_row_index, self.to_row_index);
            let self_to_row = cmp::max(self.from_row_index, self.to_row_index);
            
            let next_row = next_path.from_row_index;
            let next_from_col = cmp::min(next_path.from_col_index, next_path.to_col_index);
            let next_to_col = cmp::max(next_path.from_col_index, next_path.to_col_index);


            intersection = next_from_col <= self_col && next_to_col >= self_col &&
                    self_from_row <= next_row && self_to_row >= next_row;
        }

        if intersection {
            println!("CROSS_____ {self:?} vs {next_path:?}");
        }

        return intersection;

        //

        //let row_overlap = self.from_row_index <= next_path.to_row_index && self.to_row_index >= next_path.from_row_index;
        //let col_overlap = self.from_col_index <= next_path.to_col_index && self.to_col_index >= next_path.from_col_index;
        
        //row_overlap && col_overlap
    }
}
