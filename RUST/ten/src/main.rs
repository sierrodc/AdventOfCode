use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "D:\\Personal\\AdventOfCode\\DATASET\\ten\\input.txt";

    let map = read_map(filename)?;
    let mut total_distinct_targets = 0;
    let mut total_distinct_routes = 0;
    for pos in map.zero_positions() {
        // todo
        let mut final_distinct_positions: HashSet<Point> = HashSet::new();
        let mut final_position: Vec<Point> = Vec::new();
        let mut queue: VecDeque<Cell> = VecDeque::new();
        queue.push_back(Cell {
            point: pos,
            value: 0
        });

        while let Some(cell) = queue.pop_front() {
            for next_cell in map.move_next(&cell) {
                if next_cell.value == 9 {
                    final_distinct_positions.insert(next_cell.point);
                    final_position.push(next_cell.point);
                } else {
                    queue.push_back(next_cell);
                }
            }
        }

        total_distinct_targets += final_distinct_positions.len();
        total_distinct_routes += final_position.len();
    }

    println!("Distinct target per starting point: {total_distinct_targets}, Distinct complete routes: {total_distinct_routes} in: {:.2?}", init_ts.elapsed());

    Ok(())
}

fn read_map(filename: &str) -> Result<Map, io::Error> {
    let file = File::open(filename)?;
    let reader = std::io::BufReader::new(file);

    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let int_row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        matrix.push(int_row);
    }
    let rows = matrix.len();
    let cols = matrix[0].len();

    Ok(Map {
        data: matrix,
        rows: rows,
        cols: cols,
    })
}

struct Map {
    data: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn zero_positions(&self) -> impl Iterator<Item = Point> + '_ {
        self.data.iter().enumerate().flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_index, &item)| {
                    if item == 0 {
                        Some(Point {
                            row_idx: row_index,
                            col_idx: col_index,
                        })
                    } else {
                        None
                    }
                })
        })
    }

    fn move_next(&self, cell: &Cell) -> Vec<Cell> {
        let mut positions: Vec<Cell> = Vec::new();

        // Check left 
        if cell.point.col_idx > 0 && self.data[cell.point.row_idx][cell.point.col_idx - 1] == cell.value + 1 {
            positions.push(Cell {
                point: Point {
                    row_idx: cell.point.row_idx,
                    col_idx: cell.point.col_idx - 1,
                },
                value: cell.value + 1,
            });
        } 
        
        // Check right 
        if cell.point.col_idx < self.cols - 1 && self.data[cell.point.row_idx][cell.point.col_idx + 1] == cell.value + 1 {
            positions.push(Cell {
                point: Point {
                    row_idx: cell.point.row_idx,
                    col_idx: cell.point.col_idx + 1,
                },
                value: cell.value + 1,
            });
        } 
        
        
        // Check up 
        if cell.point.row_idx > 0 && self.data[cell.point.row_idx - 1][cell.point.col_idx] == cell.value + 1 {
            positions.push(Cell {
                point: Point {
                    row_idx: cell.point.row_idx - 1,
                    col_idx: cell.point.col_idx,
                },
                value: cell.value + 1,
            });
        } 

        // Check up 
        if cell.point.row_idx < self.rows - 1 && self.data[cell.point.row_idx + 1][cell.point.col_idx] == cell.value + 1 {
            positions.push(Cell {
                point: Point {
                    row_idx: cell.point.row_idx + 1,
                    col_idx: cell.point.col_idx,
                },
                value: cell.value + 1,
            });
        } 

        return positions;
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    row_idx: usize,
    col_idx: usize,
}

struct Cell {
    point: Point,
    value: u32,
}