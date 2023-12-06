use std::{fs::File, io::BufReader};
use std::io::prelude::*;
use regex::Regex;

fn compile_regex(encoding: Vec<&str> ) -> Vec<Regex> {
    let mut compiled: Vec<Regex> = vec![];

    for statement in encoding {
        compiled.push(Regex::new(statement).unwrap());
    }
    compiled
}
fn replace_words(contents: &mut String, compiled_regex: Vec<Regex>, replacements: Vec<&str>) -> String{
    let mut new_str: String = contents.clone();
    for (idx, stmt) in compiled_regex.iter().enumerate(){
        new_str = stmt.replace_all(&new_str, replacements[idx]).to_string();
    }
    new_str
}

fn main() -> std::io::Result<()> {
let uncompiled_regex: Vec<&str> = vec![(r"(one)"), (r"(two)"), (r"(three)"), (r"(four)"), (r"(five)"), (r"(six)"), (r"(seven)"), (r"(eight)"), (r"(nine)")];
let replacements: Vec<&str> = vec!["o1e", "t2o", "t3e", "f4r", "f5e", "s6x", "s7n", "e8t", "n9e"];
let compiled_regex = compile_regex(uncompiled_regex);
let file = File::open("input.txt")?;
let mut buf_reader = BufReader::new(file);
let mut contents = String::new();
buf_reader.read_to_string(&mut contents)?;
let old_contents = contents.clone();
contents = replace_words(&mut contents, compiled_regex, replacements);
assert_ne!(contents, old_contents);
let mut total: i32 = 0;

let mut line_counter: i32 = 0;
for line in contents.clone().split("\n") {
    line_counter+= 1;
    if line.len() == 0{
        break;
    }
    let new_line: Vec<i32> = line.chars().map(|s| s.to_string().parse::<i32>().unwrap_or_default()).filter(|&x| x != 0).collect();
    let mut filtered_line: String = "".to_string();
    for item in new_line{
        filtered_line += &item.to_string();
    }
    //println!("Line #{line_counter}: {filtered_line}");
    let first = filtered_line.chars().nth(0).unwrap();
    let sec: char = filtered_line.chars().rev().nth(0).unwrap();
    let sum: i32 = (first.to_digit(10).unwrap() * 10 + sec.to_digit(10).unwrap()) as i32;
    total += sum;
    //println!("TEST: First: {first}, sec: {sec}, sum: {sum}"); 
}
println!("Total: {total}");
Ok(())  
}


