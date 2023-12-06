#[derive(Debug, Clone)]
pub struct Scratchy {
    pub id: usize,
    winners: Vec<i64>,
    plays: Vec<i64>,
}

impl Scratchy {
    pub fn new(data: &str) -> Scratchy {
        let first_split: Vec<&str> = data.split(":").collect();

        let id_bit = first_split[0];
        let id: Vec<&str> = id_bit.split(" ").collect();

        let winners_and_plays: Vec<&str> = first_split[1].split("|").collect();
        let raw_winners: Vec<&str> = winners_and_plays[0].trim().split(" ").collect();
        let mut winners: Vec<i64> = Vec::new();

        let _: Vec<_> = raw_winners
            .into_iter()
            .map(|winner| {
                if winner != "" {
                    winners.push(String::from(winner).parse::<i64>().unwrap());
                }
            })
            .collect();

        let raw_plays: Vec<&str> = winners_and_plays[1].trim().split(" ").collect();
        let mut plays: Vec<i64> = Vec::new();

        let _: Vec<_> = raw_plays
            .into_iter()
            .map(|play| {
                if play != "" {
                    plays.push(String::from(play).parse::<i64>().unwrap());
                }
            })
            .collect();

        Scratchy {
            id: String::from(id[id.len() - 1]).parse::<usize>().unwrap(),
            winners,
            plays,
        }
    }

    pub fn wins(&self) -> i64 {
        let mut value = 0;

        let _: Vec<_> = self
            .winners
            .iter()
            .map(|win| {
                if self.plays.contains(win) {
                    value += 1;
                }
            })
            .collect();
        value
    }

    pub fn value(&self) -> i64 {
        let mut value = 0;

        let _: Vec<_> = self
            .winners
            .iter()
            .map(|win| {
                if self.plays.contains(win) {
                    if value == 0 {
                        value = 1;
                    } else {
                        value = value << 1;
                    }
                }
            })
            .collect();

        value
    }
}
