use std::{collections::HashMap, io::BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "X:\\Personal\\AdventOfCode\\DATASET\\one\\input.txt";
    let filepath = std::path::Path::new(filename);

    if !filepath.exists() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("File {filename} doesn't exist"))));
    }
    let file = std::fs::File::open(&filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("   ").collect();

        first.push(parts[0].parse()?);
        second.push(parts[1].parse()?);
    }

    first.sort();
    second.sort();

    let result: i32 = first.iter().zip(second.iter()).map(|(a, b)| (a-b).abs()).sum();

    // lists are sorted => this code can be optimized
    let mut first_occurrences: HashMap<i32, i32> = HashMap::new();
    let mut second_occurrences: HashMap<i32, i32> = HashMap::new();
    for item in first {
        *first_occurrences.entry(item).or_insert(0) += 1;
    }
    for item in second {
        *second_occurrences.entry(item).or_insert(0) += 1;
    }

    let result2: i32 = first_occurrences.iter().map(|(key, val)| key * val * second_occurrences.get(key).unwrap_or(&0)).sum();

    println!("First distance: {result}");
    println!("Second distance: {result2}");

    Ok(())
}
