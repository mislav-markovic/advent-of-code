use std::str::FromStr;

use crate::day_exec::DayExecutor;

pub struct Day2;
impl DayExecutor for Day2 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Total score following strategy guide with choices to play: {}",
            solve_part1(&input)
        ))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Total score following strategy guide with desired outcomes: {}",
            solve_part2(&input)
        ))
    }
}

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.parse::<RoundWithChoices>()
                .expect("Failed to parse line as rock-paper-scissors round")
                .0
        })
        .map(|r| r.round_score())
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.parse::<RoundWithOutcome>()
                .expect("Failed to parse round with provided outcome")
                .0
        })
        .map(|r| r.round_score())
        .sum()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

struct OpponentChoice {
    choice: HandShape,
}

impl OpponentChoice {
    fn new(choice: HandShape) -> Self {
        Self { choice }
    }
}

struct OpponentChoiceParseError;
impl FromStr for OpponentChoice {
    type Err = OpponentChoiceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.len() > 1 {
            Err(Self::Err {})
        } else {
            match trimmed.chars().next().unwrap() {
                'A' => Ok(OpponentChoice::new(HandShape::Rock)),
                'B' => Ok(OpponentChoice::new(HandShape::Paper)),
                'C' => Ok(OpponentChoice::new(HandShape::Scissors)),
                _ => Err(Self::Err {}),
            }
        }
    }
}

struct PlayerChoice {
    choice: HandShape,
}

impl PlayerChoice {
    fn new(choice: HandShape) -> Self {
        Self { choice }
    }
}

struct PlayerChoiceParseError;
impl FromStr for PlayerChoice {
    type Err = PlayerChoiceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.len() > 1 {
            Err(Self::Err {})
        } else {
            match trimmed.chars().next().unwrap() {
                'X' => Ok(PlayerChoice::new(HandShape::Rock)),
                'Y' => Ok(PlayerChoice::new(HandShape::Paper)),
                'Z' => Ok(PlayerChoice::new(HandShape::Scissors)),
                _ => Err(Self::Err {}),
            }
        }
    }
}

enum RoundOutcome {
    Loss,
    Draw,
    Win,
}

struct RoundOutcomeParseError;
impl FromStr for RoundOutcome {
    type Err = RoundOutcomeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        if trimmed.len() > 1 {
            Err(RoundOutcomeParseError {})
        } else {
            match trimmed.chars().next().unwrap() {
                'X' => Ok(RoundOutcome::Loss),
                'Y' => Ok(RoundOutcome::Draw),
                'Z' => Ok(RoundOutcome::Win),
                _ => Err(RoundOutcomeParseError),
            }
        }
    }
}

struct Round {
    player: PlayerChoice,
    opponent: OpponentChoice,
    player_outcome: RoundOutcome,
}

impl Round {
    fn new(player: PlayerChoice, opponent: OpponentChoice) -> Self {
        let player_outcome = play_round(&player, &opponent);
        Self {
            player,
            opponent,
            player_outcome,
        }
    }

    fn from_outcome(opponent: OpponentChoice, player_outcome: RoundOutcome) -> Self {
        let player = determine_choice_for_outcome(&opponent, &player_outcome);
        Self {
            opponent,
            player,
            player_outcome,
        }
    }

    fn round_score(&self) -> u32 {
        let shape_choice_score = match self.player.choice {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        };

        let outcome_score = match self.player_outcome {
            RoundOutcome::Loss => 0,
            RoundOutcome::Draw => 3,
            RoundOutcome::Win => 6,
        };

        shape_choice_score + outcome_score
    }
}

fn determine_choice_for_outcome(
    opponent_choice: &OpponentChoice,
    desired_outcome: &RoundOutcome,
) -> PlayerChoice {
    let player_choice = match (desired_outcome, opponent_choice.choice) {
        (RoundOutcome::Draw, choice) => choice,
        (RoundOutcome::Win, choice) => win(&choice),
        (RoundOutcome::Loss, choice) => lose(&choice),
    };

    PlayerChoice::new(player_choice)
}

fn win(choice: &HandShape) -> HandShape {
    match choice {
        HandShape::Rock => HandShape::Paper,
        HandShape::Paper => HandShape::Scissors,
        HandShape::Scissors => HandShape::Rock,
    }
}

fn lose(choice: &HandShape) -> HandShape {
    match choice {
        HandShape::Rock => HandShape::Scissors,
        HandShape::Paper => HandShape::Rock,
        HandShape::Scissors => HandShape::Paper,
    }
}

fn play_round(player: &PlayerChoice, opponent: &OpponentChoice) -> RoundOutcome {
    match (player.choice, opponent.choice) {
        (p, o) if p == o => RoundOutcome::Draw,
        (HandShape::Rock, HandShape::Scissors) => RoundOutcome::Win,
        (HandShape::Paper, HandShape::Rock) => RoundOutcome::Win,
        (HandShape::Scissors, HandShape::Paper) => RoundOutcome::Win,
        _ => RoundOutcome::Loss,
    }
}

#[derive(Debug)]
struct RoundParseError;

struct RoundWithChoices(Round);

impl FromStr for RoundWithChoices {
    type Err = RoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent_choice_str, player_choice_str) = s.trim().split_once(' ').unwrap();

        let opponent_choice = opponent_choice_str
            .parse::<OpponentChoice>()
            .map_err(|_| Self::Err {})?;

        let player_choice = player_choice_str
            .parse::<PlayerChoice>()
            .map_err(|_| Self::Err {})?;

        Ok(Self {
            0: Round::new(player_choice, opponent_choice),
        })
    }
}
struct RoundWithOutcome(Round);

impl FromStr for RoundWithOutcome {
    type Err = RoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent_choice_str, desired_outcome_str) = s.trim().split_once(' ').unwrap();

        let opponent_choice = opponent_choice_str
            .parse::<OpponentChoice>()
            .map_err(|_| Self::Err {})?;

        let player_outcome = desired_outcome_str
            .parse::<RoundOutcome>()
            .map_err(|_| Self::Err {})?;

        Ok(Self {
            0: Round::from_outcome(opponent_choice, player_outcome),
        })
    }
}
