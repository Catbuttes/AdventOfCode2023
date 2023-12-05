use std::fs;

fn main() {
    let max_red: i64 = 12;
    let max_green: i64 = 13;
    let max_blue: i64 = 14;

    let mut possible_sum: i64 = 0;
    let mut power_sum: i64 = 0;

    //let data = fs::read_to_string("test").unwrap();
    let data = fs::read_to_string("input").unwrap();

    let data_lines: Vec<&str> = data.as_str().split("\n").collect();

    let _: Vec<_> = data_lines
        .iter()
        .map(|line| {
            if line == &"" {
                return;
            }
            let a = Game::new(line);
            if a.is_valid(max_red, max_green, max_blue) {
                //println!("{:?}", a);
                possible_sum += a.id;
            }
            power_sum += a.power();
            println!("{:?}, power: {:}", a, a.power());
        })
        .collect();

    println!("Possible Game Sum: {possible_sum}");
    println!("Game Power Sum: {power_sum}");
}

#[derive(Debug)]
struct Game {
    id: i64,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: i64,
    blue: i64,
    green: i64,
}

impl Game {
    pub fn new(data_line: &str) -> Game {
        let first_split: Vec<&str> = data_line.split(":").collect();
        let game_number: Vec<&str> = first_split[0].split(" ").collect();
        let id: i64 = game_number[1].parse::<i64>().unwrap();

        let rounds: Vec<&str> = first_split[1].split(";").collect();

        let mut processed_rounds: Vec<Round> = Vec::new();

        let _: Vec<_> = rounds
            .iter()
            .map(|r| processed_rounds.push(Round::new(r)))
            .collect();

        Game {
            id,
            rounds: processed_rounds,
        }
    }

    pub fn is_valid(&self, max_red: i64, max_green: i64, max_blue: i64) -> bool {
        let mut is_valid = true;

        let _: Vec<_> = self
            .rounds
            .iter()
            .map(|round| {
                if !round.is_valid(max_red, max_green, max_blue) {
                    is_valid = false;
                }
            })
            .collect();

        is_valid
    }

    pub fn power(&self) -> i64 {
        let mut red: i64 = 0;
        let mut green: i64 = 0;
        let mut blue: i64 = 0;

        let _: Vec<_> = self
            .rounds
            .iter()
            .map(|round| {
                if round.red > red {
                    red = round.red;
                }
                if round.green > green {
                    green = round.green;
                }
                if round.blue > blue {
                    blue = round.blue;
                }
            })
            .collect();

        red * green * blue
    }
}

impl Round {
    pub fn new(set_data: &str) -> Round {
        let set_parts: Vec<&str> = set_data.trim().split(",").collect();

        let mut red: i64 = 0;
        let mut blue: i64 = 0;
        let mut green: i64 = 0;

        let _: Vec<_> = set_parts
            .iter()
            .map(|part| match part {
                color if color.contains("red") => {
                    let pull: Vec<&str> = color.trim().split(" ").collect();
                    red = pull[0].parse::<i64>().unwrap();
                }

                color if color.contains("blue") => {
                    let pull: Vec<&str> = color.trim().split(" ").collect();
                    blue = pull[0].parse::<i64>().unwrap();
                }
                color if color.contains("green") => {
                    let pull: Vec<&str> = color.trim().split(" ").collect();
                    green = pull[0].parse::<i64>().unwrap();
                }
                &_ => {}
            })
            .collect();

        Round { red, blue, green }
    }

    pub fn is_valid(&self, max_red: i64, max_green: i64, max_blue: i64) -> bool {
        if self.red <= max_red && self.green <= max_green && self.blue <= max_blue {
            return true;
        }

        false
    }
}
