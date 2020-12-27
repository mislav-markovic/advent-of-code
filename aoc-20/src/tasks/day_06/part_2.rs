use super::Group;

pub(super) fn sum_of_answeres_where_everyone_said_yes(groups: &[Group]) -> usize {
  groups
    .iter()
    .map(|grp| grp.which_questions_everyone_answered_yes().len())
    .sum()
}
