use std::usize;
use std::{io::BufRead};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "D:\\Personal\\AdventOfCode\\DATASET\\two\\input.txt";
    let filepath = std::path::Path::new(filename);

    let file = std::fs::File::open(&filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut correct_reports = 0;
    let mut correct_reports_considering_dampener = 0;

    let mut levels: Vec<i8> = Vec::new();
    for report in reader.lines() {
        levels.clear();
        let report = report?;
        for level_str in report.split(' ') {
            levels.push(level_str.parse()?);
        }

        if is_report_correct(&levels, Option::Some(usize::MAX)) {
            correct_reports+=1;
        }
        if is_report_correct(&levels, None) {
            correct_reports_considering_dampener+=1;
        }
    }

    println!("Correct reports: {correct_reports}, {correct_reports_considering_dampener} considering Dampener in {:.2?}", init_ts.elapsed());
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
