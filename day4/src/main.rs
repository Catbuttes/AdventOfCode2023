use std::{collections::HashMap, fs};

mod scratchy;
use scratchy::Scratchy;

fn main() {
    let data = load_data();
    let games = setup_games(data.as_str());
    let points = get_total(games.clone());
    let cards = count_cards(games.clone());
    println!("Cards: {:?}", cards);
    println!("Points: {:?}", points);
}

fn load_data() -> String {
    fs::read_to_string("input").unwrap()
}

fn setup_games(data: &str) -> Vec<Scratchy> {
    let game_data: Vec<&str> = data.split("\n").collect();
    let mut games: Vec<Scratchy> = Vec::new();

    let _: Vec<_> = game_data
        .into_iter()
        .map(|line| {
            if line != "" {
                games.push(Scratchy::new(line))
            }
        })
        .collect();

    games
}

fn get_total(games: Vec<Scratchy>) -> i64 {
    let mut accumulator = 0;

    let _: Vec<_> = games.iter().map(|g| accumulator += g.value()).collect();

    accumulator
}

fn count_cards(games: Vec<Scratchy>) -> i64 {
    let mut cards: HashMap<usize, i64> = HashMap::new();
    let mut card_total: i64 = 0;
    //set up one of each card
    for i in 1..games.len() + 1 {
        cards.insert(i, 1);
    }

    //println!("{:?}", games);

    for i in 0..games.len() {
        let card = games[i].clone();
        println!("Playing card {:?}", card);
        let plays: i64 = cards.get(&card.id).unwrap().to_owned();

        for _ in 0..plays {
            card_total += 1;
            let wins = card.wins();

            //println!("Card {0}, Wins: {wins}", card.id);

            for j in 1..wins + 1 {
                let offset = usize::try_from(j).unwrap();
                let index: usize = card.id + offset;
                //println!("Adding to card {index}");
                let counts = cards.get(&index).unwrap();
                cards.insert(index, counts + 1);
            }
        }
    }

    card_total
}

#[cfg(test)]
mod tests {
    use crate::{count_cards, get_total, setup_games};

    use std::fs;

    #[test]
    fn test() {
        let data = fs::read_to_string("test").unwrap();
        let games = setup_games(data.as_str());
        let points = get_total(games.clone());
        let cards = count_cards(games.clone());
        println!("Points: {:?}", points);
        println!("Cards: {:?}", cards);
        ()
    }
}
