use std::{
    fmt::{self, Formatter},
    fs::File,
    io::{prelude::*, BufReader},
    vec,
};

struct Game {
    red: Vec<i32>,
    green: Vec<i32>,
    blue: Vec<i32>,
}

impl Game {
    fn zero_self(&mut self) -> () {
        self.red = vec![];
        self.green = vec![];
        self.blue = vec![];
    }
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
            game_to_check.zero_self();
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
            /*if !THRESHOLD_VALS.compare(&game_to_check) {
                bad_games = format!("{},{}", bad_games, idx + 1);
                should_skip = true;
                break;
            }*/
        }
        /*if !should_skip {
            sum_of_ids += (idx as i32) + 1;
        }*/
        // Calculate powers
        let max_red: i32 = game_to_check.red.into_iter().max().unwrap_or(1);
        let max_green: i32 = game_to_check.green.into_iter().max().unwrap_or(1);
        let max_blue: i32 = game_to_check.blue.into_iter().max().unwrap_or(1);
        //println!("Max red val: {max_red}, Max green val: {max_green}, Max blue val: {max_blue}");
        println!("Combined power: {}", max_red * max_green * max_blue);
        max_power_sum += max_red * max_green * max_blue;
    }
    //println!("Bad games: {}", &bad_games[1..]);
    println!("Max power sum: {max_power_sum}");
    Ok(())
}
