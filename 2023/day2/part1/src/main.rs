use std::{
    fmt::{self, Formatter},
    fs::File,
    io::{prelude::*, BufReader},
};

struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn compare(&self, other: &Game) -> bool {
        if other.red > self.red || other.green > self.green || other.blue > self.blue {
            false
        } else {
            true
        }
    }

    fn zero_self(&mut self) -> () {
        self.red = 0;
        self.green = 0;
        self.blue = 0;
    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Red: {}, Green: {}, Blue: {}",
            self.red, self.green, self.blue
        )
    }
}

const THRESHOLD_VALS: Game = Game {
    red: 12,
    green: 13,
    blue: 14,
};
fn main() -> std::io::Result<()> {
    let mut bad_games: String = "".to_string();
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut sum_of_ids: i32 = 0;
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let games = contents.split("\n");
    for (idx, val) in games.enumerate() {
        let mut should_skip: bool = false;
        if val.is_empty(){
            break;
        }
        let find_colon = val.find(':').unwrap();
        let mut game_to_check = Game {
            red: 0,
            green: 0,
            blue: 0,
        };
        let trimmed_line = &val.to_string()[find_colon + 1..];
        let individual_hands = trimmed_line.split(";");
        for item in individual_hands {
            if should_skip {
                break;
            }
            game_to_check.zero_self();
            let hands = item.split(",");
            for obj in hands {
                let mut hand = obj.split_ascii_whitespace();
                let color = hand.next_back().unwrap();
                let amount = hand.next().unwrap().to_string().parse::<i32>().unwrap();
                match color {
                    "red" => game_to_check.red += amount,
                    "green" => game_to_check.green += amount,
                    "blue" => game_to_check.blue += amount,
                    _ => (),
                };
            }
            if !THRESHOLD_VALS.compare(&game_to_check) {
                bad_games = format!("{},{}", bad_games, idx + 1);
                should_skip = true;
                break;
            }
        }
        if !should_skip {
            sum_of_ids += (idx as i32) + 1;
        }
    }
    println!("{}", "-".repeat(bad_games.len()));
    println!("Bad games: {}", &bad_games[1..]);
    println!("ID Sum: {sum_of_ids}");
    Ok(())
}
