use super::Game;

pub(super) fn number_played_at_turn(mut game: Game, number: &usize) -> usize {
  while game.current_turn < *number {
    game.next_turn()
  }
  game.last_number
}
