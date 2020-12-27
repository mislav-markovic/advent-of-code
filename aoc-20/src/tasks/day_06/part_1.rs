use super::Group;

pub(super) fn sum_of_group_yes_answers(groups: &[Group]) -> usize {
  groups
    .iter()
    .map(|grp| grp.which_questions_anyone_answered_yes().len())
    .sum()
}
