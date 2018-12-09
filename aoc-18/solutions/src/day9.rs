use crate::input_reader;

struct Circle {
    marbles: Vec<usize>,
    players: Vec<usize>,
    last_marble: usize,
    current_marble: usize,
    player_count: usize,
}

impl Circle {
    fn new(last_marble: usize, player_count: usize) -> Circle {
        Circle {
            marbles: Vec::with_capacity(last_marble),
            players: vec![0; player_count],
            last_marble,
            current_marble: 0,
            player_count,
        }
    }

    fn play(&mut self, marble: usize, player: usize) {
        if self.marbles.is_empty() {
            self.marbles.push(marble);
        } else {
            let pos = (self.current_marble + 2) % self.marbles.len();
            if marble % 23 == 0 {
                let remove_index = if self.current_marble < 7 {
                    self.marbles.len() - (7 - self.current_marble)
                } else {
                    (self.current_marble - 7) % self.marbles.len()
                };
                self.players[player] += marble;
                let removed_val = self.marbles.remove(remove_index);
                self.players[player] += removed_val;
                self.current_marble = remove_index;
            } else {
                self.marbles.insert(pos, marble);
                self.current_marble = pos;
            }
        }
    }

    fn play_all(&mut self) {
        (0..self.last_marble)
            .zip((0..self.player_count).cycle())
            .for_each(|(m, p)| self.play(m, p));
    }

    fn winning_score(&self) -> usize {
        *self.players.iter().max().unwrap_or(&0)
    }
}

fn part1(input: &str) -> usize {
    let data = input_reader::read_all(input);
    let arr = data.split_whitespace().collect::<Vec<_>>();
    let players = arr[0].parse::<usize>().unwrap();
    let marbles = arr[6].parse::<usize>().unwrap();

    let mut game = Circle::new(marbles, players);
    game.play_all();
    game.winning_score()
}

fn part2(input: &str) {
    let data = input_reader::read_all_lines(input);
}

pub fn day9() {
    let input = String::from("day9");

    println!("***Day Nine***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tWinning elfs score: {}", part1(&input));
    //println!("\t**Part Two**");
    //println!("\t\tTime needed with 5 workers: {}", part2(&input));
}

#[cfg(test)]
mod tests {}
