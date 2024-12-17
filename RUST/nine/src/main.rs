use std::collections::HashSet;
use std::hash::Hash;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "D:\\Personal\\AdventOfCode\\DATASET\\nine\\input.txt";
    let diskmap = std::fs::read_to_string(filename)?;

    // convert diskmap [2333133121414131402] to blocks [00...111...2...333. etc]
    let disk: Vec<DiskBlock> = get_disk_rapresentation(&diskmap);

    // iterate from end to beginning to fill empty spaces...
    let compacted_disk: Vec<DiskBlock> = get_compacted_disk_rapresentation_splitting_spaces(&disk);
    let compacted_disk_checksum = get_checksum(&compacted_disk);
     
    let full_file_compacted_disk: Vec<DiskBlock> = get_compacted_disk_rapresentation_moving_full_file(&disk);
    let full_file_compacted_disk_checksum = get_checksum(&full_file_compacted_disk);

    let full_file_compacted_disk_fast: Vec<DiskBlock> = get_compacted_disk_rapresentation_moving_full_file_fast(&disk);
    let full_file_compacted_disk_fast_checksum = get_checksum(&full_file_compacted_disk_fast);
    
    println!("Sum {compacted_disk_checksum}, not fragmenting fast {full_file_compacted_disk_fast_checksum}, not fragmenting optimized {full_file_compacted_disk_checksum} in: {:.2?}", init_ts.elapsed());

    Ok(())
}

fn get_checksum(disk: &Vec<DiskBlock>) -> u64 {
    let mut sum: u64 = 0;
    let mut idx: u64 = 0;
    for block in disk.iter() {
        for _rep in 0..block.repetition {
            if let Some(data) = block.data {
                sum += idx * (data as u64);
                idx += 1;
            } else {
                // empty space:
                idx += block.repetition as u64;
                break;
            }
        }
    }  

    return sum;
}

fn get_compacted_disk_rapresentation_moving_full_file(disk: &Vec<DiskBlock>) -> Vec<DiskBlock> {
    let mut compacted_disk: Vec<DiskBlock> = Vec::new();
    let mut files_moved: HashSet<&DiskBlock> = HashSet::new();

    // idea: iterate thrugh all disk blocks.
    // - if it's a space OR a previously moved file, increase available space.
    // - if it's a file
    // --- fill previous space
    // --- add current file (if not already moved)

    let mut spaces: u32 = 0;
    for disk_block in disk.iter() {
        if disk_block.data.is_none() || files_moved.contains(disk_block) { // empty
            spaces += disk_block.repetition;
        } else {
            if spaces > 0 { // try to fill space with files
                for disk_block_movable in disk.iter().rev() {
                    if disk_block_movable.data.is_none() || files_moved.contains(disk_block_movable) {
                        continue; // already moved, check next one
                    }

                    if disk_block_movable.repetition <= spaces && !disk_block_movable.data.is_none() { // is data and can be moved
                        compacted_disk.push(DiskBlock {
                            data: disk_block_movable.data,
                            repetition: disk_block_movable.repetition
                        });
                        files_moved.insert(disk_block_movable);
                        spaces -= disk_block_movable.repetition;
                    }

                    if disk_block_movable == disk_block {
                        break; //quit because block same as current one.
                    }
                }
            }

            if spaces > 0 { // no file available found
                compacted_disk.push(DiskBlock { // no files available
                    data: None,
                    repetition: spaces
                });
                spaces = 0;
            }

            if files_moved.insert(disk_block) {
                // file not moved
                compacted_disk.push(DiskBlock {
                    data: disk_block.data,
                    repetition: disk_block.repetition
                });
            }
        }

    }


    return compacted_disk;
}

fn get_compacted_disk_rapresentation_moving_full_file_fast(disk: &Vec<DiskBlock>) -> Vec<DiskBlock> {
    // create a copy to modify
    let mut compacted_disk: Vec<DiskBlock> = disk.iter().map(|d| DiskBlock { data: d.data, repetition: d.repetition }).collect();

    let mut compacted_idx = compacted_disk.len()-1;

    while 1 != compacted_idx {
        
        let data_block = compacted_disk[compacted_idx]; // same as compacted_disk[compacted_idx]
        if !data_block.data.is_none() { // let see if I can found a free space from the beginning
            // available spaces
            
            let founded_space = compacted_disk.iter().enumerate()
                .filter(|(_, blk)| blk.data.is_none())
                .filter(|(_, blk)| blk.repetition >= data_block.repetition)
                .filter(|(space_idx, _)| *space_idx < compacted_idx)
                .next();

            if let Some((space_idx, space_block)) = founded_space {
                // todo
                if space_block.repetition == data_block.repetition {
                    // move there. no need to expand space because end space is no more usable.
                    compacted_disk[space_idx] = DiskBlock {
                        data: data_block.data,
                        repetition: data_block.repetition
                    };
                    compacted_disk[compacted_idx] = DiskBlock {
                        data: None,
                        repetition: data_block.repetition
                    };
                    compacted_idx -= 1; // move before
                } else {
                    compacted_disk.insert(space_idx+1, DiskBlock {
                        data: None,
                        repetition: space_block.repetition - data_block.repetition
                    });
                    compacted_disk[compacted_idx+1 /*one item has been inserted before*/] = DiskBlock {
                        data: None,
                        repetition: data_block.repetition
                    };
                    compacted_disk[space_idx] = DiskBlock {
                        data: data_block.data,
                        repetition: data_block.repetition
                    };
                    
                    compacted_idx -= 0; // don't move because a new block has been inserted
                }
            } else {
                compacted_idx -= 1; //not able to find a good empty space before     
            }
        } else {
            compacted_idx -= 1; // space left where it is
        }
    }
        

    return compacted_disk;
}


fn get_compacted_disk_rapresentation_splitting_spaces(disk: &Vec<DiskBlock>) -> Vec<DiskBlock> {
    let mut compacted_disk: Vec<DiskBlock> = Vec::new();

    let mut start_idx: usize = 0;
    let mut end_idx = disk.len();
    let mut remaining_end_block = DiskBlock  {
        data: None,
        repetition: 0
    };

    while start_idx != end_idx {
        
        let start_block = &disk[start_idx];
        if let Some(start_data) = start_block.data {
            // add data from beginning of the disk
            compacted_disk.push(DiskBlock {
                data: Some(start_data),
                repetition: start_block.repetition
            });
            start_idx += 1;
        } else {
            // available spaces
            let mut spaces = start_block.repetition;

            // fill spaces with previous end block:
            while spaces > 0 {

                while remaining_end_block.repetition == 0 {
                    end_idx = end_idx - 1;
                    if end_idx == start_idx { break; }

                    let end_block = &disk[end_idx];
                    if let Some(end_data) = end_block.data {
                        remaining_end_block = DiskBlock {
                            data: Some(end_data),
                            repetition: end_block.repetition
                        };
                    } 
                }
                
                if remaining_end_block.repetition > spaces { // end block fill spaces
                    compacted_disk.push(DiskBlock {
                        data: remaining_end_block.data,
                        repetition: spaces
                    });
                    remaining_end_block.repetition -= spaces;
                    spaces = 0;
                } else { // end block doesn't fill spaces
                    compacted_disk.push(DiskBlock {
                        data: remaining_end_block.data,
                        repetition: remaining_end_block.repetition
                    });
                    spaces -= remaining_end_block.repetition;
                    remaining_end_block.repetition = 0;
                }
            }
            
            if spaces == 0 {
                start_idx += 1;
            }
        }

    }

    if remaining_end_block.repetition > 0 {
        compacted_disk.push(DiskBlock {
            data: remaining_end_block.data,
            repetition: remaining_end_block.repetition
        });
    }

    return compacted_disk;
}

fn get_disk_rapresentation(diskmap: &str) -> Vec<DiskBlock> {
    let mut disk : Vec<DiskBlock> = Vec::new();
    let mut is_space = false;
    let mut block_index: u32 = 0;
    for char in diskmap.chars() {
        let number: u32 = char.to_digit(10).unwrap();

        if is_space {
            disk.push(DiskBlock {
                data: None,
                repetition: number
            });
        } else {
            disk.push(DiskBlock {
                data: Some(block_index),
                repetition: number
            });
            block_index += 1;
        }

        is_space = !is_space;
    }

    return disk;
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct DiskBlock {
    data: Option<u32>,
    repetition: u32
}