use std::{
    fs::File,
    io::{prelude::*, BufReader},
    vec,
};

struct Game {
    red: Vec<i32>,
    green: Vec<i32>,
    blue: Vec<i32>,
}
fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut max_power_sum: i32 = 0;
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let games = contents.split("\n");
    for val in games {
        if val.is_empty() {
            break;
        }
        let find_colon = val.find(':').unwrap();
        let mut game_to_check = Game {
            red: vec![],
            green: vec![],
            blue: vec![],
        };
        let trimmed_line = &val.to_string()[find_colon + 1..];
        let individual_hands = trimmed_line.split(";");
        for item in individual_hands {
            let hands = item.split(",");
            for obj in hands {
                let mut hand = obj.split_ascii_whitespace();
                let color = hand.next_back().unwrap();
                let amount = hand.next().unwrap().to_string().parse::<i32>().unwrap();
                match color {
                    "red" => game_to_check.red.push(amount),
                    "green" => game_to_check.green.push(amount),
                    "blue" => game_to_check.blue.push(amount),
                    _ => (),
                };
            }
        }

        let mut max_red: i32 = game_to_check.red.into_iter().max().unwrap_or(1);
        let mut max_green: i32 = game_to_check.green.into_iter().max().unwrap_or(1);
        let mut max_blue: i32 = game_to_check.blue.into_iter().max().unwrap_or(1);
        //println!("Max red val: {max_red}, Max green val: {max_green}, Max blue val: {max_blue}");
        //println!("Combined power: {}", max_red * max_green * max_blue);
        if max_red == 0 {
            max_red = 1;
        }
        if max_green == 0 {
            max_green = 1;
        }
        if max_blue == 0 {
            max_blue = 1;
        }
        max_power_sum += max_red * max_green * max_blue;
    }
    println!("Max power sum: {max_power_sum}");
    Ok(())
}
