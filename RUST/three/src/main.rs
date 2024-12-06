use std::io::Read;
use std::time::Instant;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "D:\\Personal\\AdventOfCode\\DATASET\\three\\input.txt";
    let filepath = std::path::Path::new(filename);
    let mut file_content = String::new();
    
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let file = std::fs::File::open(&filepath)?;
    let mut reader = std::io::BufReader::new(file);
    reader.read_to_string(&mut file_content)?;


    let mut total = 0;
    let mut total_enabled = 0;

    total += get_multiplications(&re, file_content.as_str())?;

    for enabled_range in file_content.split("do()") {
        let until_disabled = enabled_range.split("don't()").next().unwrap();
        total_enabled += get_multiplications(&re, until_disabled)?;
    }

    println!("Total: {total}, enabled {total_enabled} in {:.2?}", init_ts.elapsed());
    Ok(())
}

fn get_multiplications(regex: &Regex, code: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut total = 0;

    for capture in regex.captures_iter(code) {
        let (_, [f1_str, f2_str]) = capture.extract();
        let f1: i32 = f1_str.parse()?;
        let f2: i32 = f2_str.parse()?;
        total += f1 * f2;
    }

    return Ok(total);
}
