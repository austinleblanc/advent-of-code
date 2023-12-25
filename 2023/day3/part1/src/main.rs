use std::{fs::File, io::{BufReader, prelude::*}};
use regex::Regex;

fn locate_number_indices(input: &str) -> Vec<usize> {
    let re = Regex::new(r"\d+").unwrap();
    let mut indices: Vec<usize> = Vec::new();
    for cap in re.captures_iter(input) {
        indices.push(cap.get(0).unwrap().start());
    }
    indices
}

fn check_adjacent(input: &str, index: usize, valid_chars: [char; 10], line_length: usize) -> bool {
    
    //println!("Index: {index}, Line length: {line_length}");
    if index as i32 - line_length as i32 >= 0 {
        let above_mid = index - line_length - 1;
        if (above_mid - 1) as i32  >= 0 {
            if valid_chars.contains(&input.chars().nth(above_mid - 1).unwrap()) {
                return true;
            }
        }
        if valid_chars.contains(&input.chars().nth(above_mid).unwrap()) {
            return true;
        }
        if above_mid + 1 < input.len() {
            if valid_chars.contains(&input.chars().nth(above_mid + 1).unwrap()) {
                return true;
            }
        }
    }
    if (index - 1) as i32 >= 0 {
        if valid_chars.contains(&input.chars().nth(index - 1).unwrap()) {
            return true;
        }
    }
    if index + 1 < input.len()  {
        if valid_chars.contains(&input.chars().nth(index + 1).unwrap()) {
            return true;
        }
    }
    if index + line_length < input.len() {
        //println!("A line exists below index!");
        let below_mid = index + line_length + 1;
        if below_mid - 1 <= input.len() {
            //println!("Left is valid");
            if valid_chars.contains(&input.chars().nth(below_mid - 1).unwrap()) {
                return true;
            }
        }
        if valid_chars.contains(&input.chars().nth(below_mid).unwrap()) {
            //println!("Mid is valid");
            return true;
        }
        if below_mid + 1 <= input.len() {
            //println!("Right is valid, current index: {index}, below mid index: {below_mid}, input length: {}, char at index: {}", input.len(), input.chars().nth(below_mid + 1).unwrap());
            if valid_chars.contains(&input.chars().nth(below_mid + 1).unwrap()) {
                //println!("Validated via below_mid + 1!");
                return true;
            }
        }
    }
    //println!("No valid character found near index: {}, digit: {}", index, input.chars().nth(index).unwrap());
    false
}

fn validate_number(input: &str, index: usize, line_length: usize, valid_chars: [char; 10]) -> bool {
    //let start_index = index;
    let non_digit_regex = Regex::new(r"\.").unwrap();
    let end_index = non_digit_regex.find_at(input, index).unwrap_or(Regex::new(r"\d").unwrap().find_at(input, input.len()-2).unwrap()).start();
    //println!("Start index: {}, End index: {}, Length: {}", start_index, end_index, end_index - start_index);
    //let valid: bool = false;
    let mut working_index = index.clone();
    while working_index < end_index {
        //println!("Current character: {}", input.chars().nth(working_index).unwrap());
        if check_adjacent(input, working_index, valid_chars, line_length) == true {
            //println!("Number validated!!!!!!!!!!!");
            return true;
        }
        working_index += 1;
    }
    // TODO: Iterate from index to end_index, checking above, below, left, right, and diagonals for valid characters.
    // Trying to minimize the amount of checks being performed.
    //println!("Invalid number!");
    false
}


fn main() -> std::io::Result<()> {
    const VALID_CHARS: [char; 10] = ['#','$','%','&','*','+','-','/','=','@'];
    println!("{}", "`".repeat(100));
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut char_vec: Vec<char> = contents.chars().filter(|x| !x.is_alphabetic() && !x.is_numeric()).collect::<Vec<char>>();
    char_vec.sort();
    char_vec.dedup();
    let unfiltered_indices: Vec<usize> = locate_number_indices(&contents.clone());
    let binding = locate_number_indices(&contents);
    let line_length = contents.clone().split("\n").collect::<Vec<&str>>()[0].len();
    let number_indices: Vec<&usize> = binding.iter().filter(|&&x| validate_number(&contents, x, line_length, VALID_CHARS) == true).collect::<Vec<&usize>>();
    println!("Initial indices len: {}, Filtered indices len: {}", &unfiltered_indices.len(), &number_indices.len());
    let mut total: i32 = 0;
    for item in number_indices {
         let end_index = Regex::new(r"\D").unwrap().find_at(&contents, *item).unwrap_or(Regex::new(r"\d").unwrap().find_at(&contents, contents.len()-2).unwrap()).start();
         let substr = contents[*item..end_index].to_string();
         //println!("Number? : {}", substr);
         total += substr.trim().parse::<i32>().unwrap();
    }
    println!("Total: {}", total);
    //let char_string: String = String::from_iter(char_vec.clone().iter());

    //println!("Characters: {char_string}");

    
    

    Ok(())
}
