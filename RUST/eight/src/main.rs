use std::cmp;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::BufRead;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "D:\\Personal\\AdventOfCode\\DATASET\\eight\\input.txt";
    let filepath = std::path::Path::new(filename);

    let file = std::fs::File::open(&filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinodes: HashSet<Point> = HashSet::new();

    let mut rows: usize = 0;
    let mut columns: usize = 0;

    for (row_idx, line) in reader.lines().enumerate() {
        let line = line?;
        
        for (col_idx, frequence) in line.chars().enumerate() {
            if frequence == '.' {
                continue;
            }
            antennas.entry(frequence).or_insert(Vec::new()).push(Point{
                row_idx: row_idx,
                col_idx: col_idx
            });
        }

        rows = row_idx + 1;
        columns = line.len();
    }

    let max_size: i64 = cmp::max(rows, columns) as i64;

    // forech frequence
    for (_frequence, positions) in antennas {

        for (idx, p1) in positions.iter().enumerate() {
            for p2 in positions.iter().skip(idx+1) {
                let dx = p2.row_idx as i64 - p1.row_idx as i64;
                let dy = p2.col_idx as i64 - p1.col_idx as i64;

                
                for m in 0..max_size  {
                    if let Some(a1) = get_antinode(p1, m*dx, m*dy, rows, columns) {
                        antinodes.insert(a1);
                    } else {
                        break;
                    }
                }
                
                for m in 0..max_size  {
                    if let Some(a2) = get_antinode(p2, -m*dx, -m*dy, rows, columns) {
                        antinodes.insert(a2);
                    } else {
                        break;
                    }
                }
                
            }
        }

    }


    let antinodes_count = antinodes.len();
    println!("Antinodes {antinodes_count} in: {:.2?}", init_ts.elapsed());

    Ok(())
}

fn get_antinode(pt: &Point, dx: i64, dy: i64, rows: usize, columns: usize) -> Option<Point> {
    let x1 = pt.row_idx as i64 - dx;
    let y1 = pt.col_idx as i64 - dy;
                
    if x1 >= 0 && y1 >= 0 {
        let x1 = x1 as usize;
        let y1 = y1 as usize;
        if x1 < rows && y1 < columns {
            return Some(Point {
                row_idx: x1 as usize,
                col_idx: y1 as usize
            });
        }
    }

    return None;
}

#[derive(Hash, Eq, PartialEq)]
struct Point {
    row_idx: usize,
    col_idx: usize
}