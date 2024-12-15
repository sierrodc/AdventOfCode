use std::hash::Hash;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "D:\\Personal\\AdventOfCode\\DATASET\\nine\\input.txt";
    let diskmap = std::fs::read_to_string(filename)?;

    // convert diskmap [2333133121414131402] to blocks [00...111...2...333. etc]
    let disk: Vec<DiskBlock> = get_disk_rapresentation(&diskmap);

    // iterate from end to beginning to fill empty spaces...
    let compacted_disk: Vec<DiskBlock> = get_compacted_disk_rapresentation(&disk);

    let mut sum: u64 = 0;
    let mut idx: u64 = 0;
    for block in compacted_disk.iter() {
        for _rep in 0..block.repetition {
            sum += idx * block.data.unwrap() as u64;
            idx += 1;
        }
    }   

    println!("Sum {sum} in: {:.2?}", init_ts.elapsed());

    Ok(())
}

fn get_compacted_disk_rapresentation(disk: &Vec<DiskBlock>) -> Vec<DiskBlock> {
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

#[derive(Hash, Eq, PartialEq)]
struct DiskBlock {
    data: Option<u32>,
    repetition: u32
}