use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "D:\\Personal\\AdventOfCode\\DATASET\\five\\input.txt";
    let filepath = std::path::Path::new(filename);

    let file = std::fs::File::open(&filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    let mut lines_iterator = reader.lines();
    for rule in lines_iterator.by_ref() {
        let rule = rule?;
        if rule.len() == 0 {
            break;
        }
        let separator_index = rule.find('|').unwrap();
        let key: i32 = rule[..separator_index].parse()?;
        let value: i32 = rule[separator_index+1..].parse()?;
        
        let greater_numbers = rules.entry(key).or_insert(HashSet::new());
        greater_numbers.insert(value);
    }

    let mut total_valid:i32 = 0;
    let mut total_valid_sorted:i32 = 0;
    let mut unable_to_sort:i32 = 0;
    for updates in lines_iterator.by_ref() {
        let updates = updates?;
        let numbers: Vec<i32> = updates.split(',').map(|n|n.parse().unwrap()).collect();

        if is_update_correct(&numbers, &rules) {
            let middle_number = numbers.get(numbers.len() / 2).unwrap();
            total_valid += middle_number;
        } else if let Some(sorted_number) = sort_numbers(&numbers, &rules) {
            let middle_number = sorted_number.get(sorted_number.len() / 2).unwrap();
            total_valid_sorted += middle_number;
        } else {
            unable_to_sort += 1;
        }
    }

    println!("Valid: {total_valid}, Sorted: {total_valid_sorted}, invalid sequences: {unable_to_sort} in {:.2?}", init_ts.elapsed());
    Ok(())
}

fn is_update_correct(numbers: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> bool {
    // going to check all couples
    for (l_idx, l) in numbers.iter().enumerate().skip(1) { // from second element
        if let Some(el) = rules.get(l) {
            for g in numbers.iter().take(l_idx) { // check previous elements
                if el.contains(g) {
                    return false;
                }
            }
        }
    };

    return true;
}

fn sort_numbers(numbers: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Option<Vec<i32>> {
    let mut result: Vec<i32> = Vec::with_capacity(numbers.len());

    for new_number in numbers {
        // idx == numbers available in result
        let mut insert_idx = result.len(); // default: end of array
        if let Some(greaters) = rules.get(new_number) {
            for (r_idx, r_number) in result.iter().enumerate() {
                // if r < new_number -> insert in specific location + check if feasible.
                if greaters.contains(r_number) {
                    insert_idx = r_idx;
                    break;
                }
            }
        }

        result.insert(insert_idx, *new_number);
        // I need to check that all next elements are not < new inserted value

        for already_added_number in result.iter().skip(insert_idx+1) {
            if let Some(greaters) = rules.get(already_added_number) {
                if greaters.contains(new_number) {
                    return None;
                }
            }
        }
    }

    return Some(result);
}