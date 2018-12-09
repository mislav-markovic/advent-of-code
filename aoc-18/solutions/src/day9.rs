use crate::input_reader;
use linked_list::{Cursor, LinkedList};

struct Circle<'a> {
    players: Vec<usize>,
    last_marble: usize,
    current_marble: Cursor<'a, usize>,
    player_count: usize,
}

impl<'a> Circle<'a> {
    fn new(last_marble: usize, player_count: usize, list: &'a mut LinkedList<usize>) -> Circle<'a> {
        Circle {
            players: vec![0; player_count],
            last_marble,
            current_marble: list.cursor(),
            player_count,
        }
    }

    fn play(&mut self, marble: usize, player: usize) {
        if marble == 0 {
            self.current_marble.insert(marble);
        } else if marble % 23 == 0 {
            for _ in 0..7 {
                //skip ghost node
                if self.current_marble.peek_prev().is_none() {
                    self.current_marble.seek_backward(1)
                }
                self.current_marble.seek_backward(1);
            }
            self.players[player] += marble;
            self.players[player] += self.current_marble.remove().unwrap();
        } else {
            for _ in 0..2 {
                //skip ghost node
                if self.current_marble.peek_next().is_none() {
                    self.current_marble.seek_forward(1)
                }
                self.current_marble.seek_forward(1);
            }

            self.current_marble.insert(marble);
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
    let mut list = LinkedList::<usize>::new();

    let mut game = Circle::new(marbles, players, &mut list);
    game.play_all();
    game.winning_score()
}

fn part2(input: &str) -> usize {
    let data = input_reader::read_all(input);
    let arr = data.split_whitespace().collect::<Vec<_>>();
    let players = arr[0].parse::<usize>().unwrap();
    let marbles = arr[6].parse::<usize>().unwrap();
    let mut list = LinkedList::<usize>::new();

    let mut game = Circle::new(100 * marbles, players, &mut list);
    game.play_all();
    game.winning_score()
}

pub fn day9() {
    let input = String::from("day9");

    println!("***Day Nine***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tWinning elfs score: {}", part1(&input));
    println!("\t**Part Two**");
    println!("\t\tWinning elfs score: {}", part2(&input));
}

#[cfg(test)]
mod tests {}
