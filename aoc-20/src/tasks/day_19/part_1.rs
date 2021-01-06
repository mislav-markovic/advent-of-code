use super::{InputReader, RuleEngine};

pub(super) fn count_matches_for_zero_rule(input: InputReader) -> usize {
  let mut engine = RuleEngine::new(&input.rules);

  input
    .messages
    .into_iter()
    .filter(|msg| engine.matches(0, &msg))
    .count()
}
