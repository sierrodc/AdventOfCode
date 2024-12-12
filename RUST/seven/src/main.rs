use std::io::BufRead;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "X:\\Personal\\AdventOfCode\\DATASET\\six\\test.txt";
    let filepath = std::path::Path::new(filename);

    let file = std::fs::File::open(&filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut sum: u64 = 0;
    for line in reader.lines() {
        let line = line?;
        let (res, terms) = line.split_once(':').unwrap();

        let result: u64 = res.parse()?;
        let terms: Vec<u64> = terms.split_whitespace().map(|t| t.parse::<u64>().unwrap()).collect();


        if can_create_operation(&terms, &result) {
            sum += result;
        }
    }
    println!("Sum {sum} in: {:.2?}", init_ts.elapsed());

    Ok(())
}

fn can_create_operation(terms: &Vec<u64>, result: &u64) -> bool {
    let operations = terms.len() - 1;

    if terms.iter().sum::<u64>() == *result {
        return true;
    }
    
    let base: u64 = 2;
    let max_number = base.pow(operations as u32) - 1;
    let multiplications = 0;
    while multiplications < max_number {


    }

    return false;
}