use std::{io::BufRead, path};

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

    println!("Result: {result}");

    Ok(())
}
