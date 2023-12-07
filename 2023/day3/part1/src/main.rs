use std::{fs::File, io::{BufReader, prelude::*}};

fn main() -> std::io::Result<()> {
    println!("{}", "`".repeat(100));
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut char_vec: Vec<char> = contents.chars().filter(|x| !x.is_alphabetic() && !x.is_numeric()).collect::<Vec<char>>();
    char_vec.sort();
    char_vec.dedup();
    

    let char_string: String = String::from_iter(char_vec.clone().iter());
    println!("Characters: {char_string}");
    Ok(())
}
