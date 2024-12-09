use core::num;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::ops::Index;
use std::usize;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "X:\\Personal\\AdventOfCode\\DATASET\\five\\input.txt";
    let filepath = std::path::Path::new(filename);

    let file = std::fs::File::open(&filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut rules: HashMap<u8, HashSet<u8>> = HashMap::new();

    let mut lines_iterator = reader.lines();
    for rule in lines_iterator.by_ref() {
        let rule = rule?;
        if rule.len() == 0 {
            break;
        }
        let separator_index = rule.find('|').unwrap();
        let key: u8 = rule[..separator_index].parse()?;
        let value: u8 = rule[separator_index+1..].parse()?;
        
        let greater_numbers = rules.entry(key).or_insert(HashSet::new());
        greater_numbers.insert(value);
    }

    let mut total_valid = 0;
    for updates in lines_iterator.by_ref() {
        let updates = updates?;
        let numbers: Vec<u8> = updates.split(',').map(|n|n.parse().unwrap()).collect();

        // going to check all couples
        let mut is_valid = true;
        for (l_idx, l) in numbers.iter().enumerate().rev() {
            print!("{l_idx}");
            if let Some(el) = rules.get(l) {
                for g in numbers.iter().take(l_idx - 1) {
                    if el.contains(g) {
                        is_valid = false;
                        break;
                    }
                }

                if !is_valid {
                    break;
                }
            }
        }

        if is_valid {
            let middle_number = numbers.get(numbers.len() / 2 + 1).unwrap();
            total_valid += middle_number;
        }
    }

    println!("Correct reports: {total_valid} considering Dampener in {:.2?}", init_ts.elapsed());
    Ok(())
}

fn is_report_correct(levels: &Vec<i8>, level_idx_to_skip: Option<usize>) -> bool {
    let mut last_level: Option<&i8> = None;
    let mut record_direction: Option<bool> = Option::None;

    for (level_idx, new_level) in levels.iter().enumerate() {
        // level to skip based on Problem Dampener
        if Some(level_idx) == level_idx_to_skip {
            continue;
        }
        if last_level == None {
            last_level = Some(new_level);
            continue;
        }
        let last_level_value = last_level.unwrap();
        let delta_level = new_level - last_level_value;
        if delta_level > 3 || delta_level < -3 || delta_level == 0 {
            //Problem Dampener
            return match level_idx_to_skip {
                None => is_report_correct(levels, Some(level_idx-1)) 
                     || is_report_correct(levels, Some(level_idx))
                     || is_report_correct(levels, Some(0)),
                _ => false
            };
        }

        let current_direction = Some(new_level > last_level_value);

        if record_direction == None {
            record_direction = current_direction;
            last_level = Some(new_level);
            continue;
        }

        if record_direction != current_direction {
            return match level_idx_to_skip {
                None => is_report_correct(levels, Some(level_idx-1)) 
                     || is_report_correct(levels, Some(level_idx))
                     || is_report_correct(levels, Some(0)),
                _ => false
            };
        }

        last_level = Some(new_level);
    }

    return true;
}
