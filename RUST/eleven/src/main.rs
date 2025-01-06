use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, collections::HashMap, hash::Hash, rc::Rc, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_ts = Instant::now();
    let filename = "D:\\Personal\\AdventOfCode\\DATASET\\eleven\\input.txt";
    let line = std::fs::read_to_string(filename)?;
    let numbers: Vec<u32> = line.split(' ').map(|n| n.parse::<u32>().unwrap()).collect();
    let blinks = 25;

    let mut result = numbers.clone();
    for blink_number in 0..blinks {
        let starting_items = result.len();
        println!("{blink_number}th blink ({starting_items} items)");
        result = blink(&result);
    }
    let number_of_stones = result.len();
    println!("{number_of_stones} stones after {blinks} blinks in: {:.2?}", init_ts.elapsed());

    let extended_blinks = 75;
    let mut existing_nodes: HashMap<u32, Rc<RefCell<Node>>> = HashMap::new();
    for number in numbers.iter() {
        existing_nodes.insert(*number, Rc::new(RefCell::new(Node {
            value: *number,
            children: None
        })));
    }
    // foreach blink -> iterate over all existing nodes and expand
    for _ in 0..extended_blinks {

        let nodes_to_expand:Vec<Rc<RefCell<Node<'_>>>> = existing_nodes.values()
            .filter(|n|n.as_ref().borrow().children.is_none())
            .map(|n|n.clone())
            .collect();

        for node in nodes_to_expand {
            
            node.borrow_mut().get_mut().ExpandIfRequired(&mut existing_nodes);
        }
        
        
    }

    Ok(())
}

fn blink(numbers: &Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::with_capacity(numbers.len() * 2);

    for number in numbers {
        result.append(&mut blink_item(*number));
    } 

    return result;
}

fn blink_item(number: u32) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::with_capacity(2);
    if number == 0 {
        result.push(1);
    } else {
        let digits = number.ilog10() + 1;
        if digits % 2 == 0 {
            let split = digits / 2;
            let base: u32 = 10;
            let first_number = number / base.pow(split);
            let second_number = number % base.pow(split);
            result.push(first_number);
            result.push(second_number);
        } else {
            result.push(number * 2024);
        }
    }

    return result;
}

struct Node<'a> {
    value: u32,
    children: Option<Vec<&'a Rc<RefCell<Node<'a>>>>>
}

impl<'a> Node<'a> {
    fn ExpandIfRequired(&'a mut self, existing_nodes: &'a mut HashMap<u32, Rc<RefCell<Node<'a>>>>) {
        if self.children.is_some() {
            return; // node already expanded => no need to expand
        }
        let children_values = blink_item(self.value);
        
        let mut children: Vec<&Node> = Vec::with_capacity(children_values.len());
        for child_value in children_values.iter() {
            existing_nodes.entry(*child_value).or_insert(Rc::new(RefCell::new(Node {
                value: *child_value,
                children: None
            })));
        }
        
        self.children = Some(children_values.iter().map(|cv| existing_nodes.get(cv).unwrap()).collect());
    }
}