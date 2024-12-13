use std::io::BufRead;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "X:\\Personal\\AdventOfCode\\DATASET\\seven\\input.txt";
    let filepath = std::path::Path::new(filename);

    let file = std::fs::File::open(&filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut sum: u64 = 0;
    for line in reader.lines() {
        let line = line?;
        let (res, terms) = line.split_once(':').unwrap();

        let result: u64 = res.parse()?;
        let terms: Vec<u64> = terms.split_whitespace().map(|t| t.parse::<u64>().unwrap()).collect();


        if can_create_operation(&terms, result) {
            sum += result;
        }
    }
    println!("Sum {sum} in: {:.2?}", init_ts.elapsed());

    Ok(())
}

fn can_create_operation(terms: &Vec<u64>, result: u64) -> bool {
    let operations = terms.len() - 1;

    // consider all '+'
    if terms.iter().sum::<u64>() == result {
        return true;
    }
    
    let mut operations: Vec<char> = vec!['+'; operations];
    while !operations.iter().all(|c| *c == '|') {

        // modify operations. the first +++++ is computed before the while
        for op_idx in 0..operations.len() {
            
            if operations[op_idx] == '+' {
                operations[op_idx] = '*';
                break;
            } else if operations[op_idx] == '*' {
                operations[op_idx] = '|';
                break;
            } else if operations[op_idx] == '|' {
                operations[op_idx] = '+';
            }
        }

        let mut operation_result = terms[0];
        for (op, term) in operations.iter().zip(terms.iter().skip(1))  {
            operation_result = match op {
                '+' => operation_result + term,
                '*' => operation_result * term,
                '|' => operation_result * 10u64.pow( term.ilog10()+1) + term,
                _ => panic!("Unsopported operation")
            };

            if operation_result > result { // if already greater, continue with next operation
                break;
            }
        }
        if operation_result == result {
            return true;
        }
    }

    return false;
}