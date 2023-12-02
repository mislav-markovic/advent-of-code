use std::{collections::HashMap, fmt::Display, str::FromStr};

const INPUT: &'static str = include_str!("input2.txt");

fn main() {
    println!("Day 02, Hello, World");
    part1();
    part2();
}

struct Game {
    id: u32,
    turns: Vec<Turn>,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((game, turns)) = s.split_once(':') else {
            return Err(ParseError::new(
                "Could not split game into id and turns".to_string(),
                s.to_string(),
            ));
        };

        let Some((_, id)) = game.split_once(' ') else {
            return Err(ParseError::new(
                "Could not extract game id".to_string(),
                game.to_string(),
            ));
        };

        let id = id.parse::<u32>().map_err(|_| {
            ParseError::new(
                "Failed to parse game id as number".to_string(),
                id.to_string(),
            )
        })?;

        let turns = turns
            .split(';')
            .map(|ts| ts.trim())
            .map(|t| t.parse::<Turn>())
            .collect::<Result<Vec<_>, ParseError>>()?;

        Ok(Game::new(id, turns))
    }
}

impl Game {
    fn new(id: u32, turns: Vec<Turn>) -> Self {
        Self { id, turns }
    }

    fn is_valid_for_rules(&self, rules: &Ruleset) -> bool {
        // this is 3-pass that could be done as single-pass
        let max_red_pull = max_pull_for_cube(&self.turns, &Cube::Red);
        let max_green_pull = max_pull_for_cube(&self.turns, &Cube::Green);
        let max_blue_pull = max_pull_for_cube(&self.turns, &Cube::Blue);

        max_red_pull <= rules.rule_for(&Cube::Red)
            && max_green_pull <= rules.rule_for(&Cube::Green)
            && max_blue_pull <= rules.rule_for(&Cube::Blue)
    }

    fn game_ruleset(&self) -> Ruleset {
        let max_red_pull = max_pull_for_cube(&self.turns, &Cube::Red);
        let max_green_pull = max_pull_for_cube(&self.turns, &Cube::Green);
        let max_blue_pull = max_pull_for_cube(&self.turns, &Cube::Blue);
        let rules = vec![
            Rule::new(Cube::Red, max_red_pull),
            Rule::new(Cube::Green, max_green_pull),
            Rule::new(Cube::Blue, max_blue_pull),
        ];

        Ruleset::new(rules)
    }
}

fn max_pull_for_cube(turns: &[Turn], cube: &Cube) -> u32 {
    turns
        .iter()
        .flat_map(|t| t.pulls.iter().filter(|p| p.cube == *cube))
        .map(|p| p.count)
        .max()
        .unwrap_or(0)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Cube {
    Red,
    Green,
    Blue,
}

impl FromStr for Cube {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Cube::Blue),
            "green" => Ok(Cube::Green),
            "red" => Ok(Cube::Red),
            _ => Err(ParseError::new(
                "Could not parse cube".to_string(),
                s.to_string(),
            )),
        }
    }
}

#[derive(Debug)]
struct ParseError {
    msg: String,
    value: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse Error on '{val}'! {msg}",
            msg = self.msg,
            val = self.value
        )
    }
}

impl ParseError {
    fn new(msg: String, value: String) -> Self {
        Self { msg, value }
    }
}

struct Pull {
    cube: Cube,
    count: u32,
}

impl FromStr for Pull {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((count, cube)) = s.split_once(' ') {
            let count = count.parse::<u32>().map_err(|_| {
                ParseError::new(
                    "Could not parse Pull count as number".to_string(),
                    count.to_string(),
                )
            })?;
            let cube = cube.parse::<Cube>()?;

            Ok(Pull::new(cube, count))
        } else {
            Err(ParseError::new(
                "Could not parse pull since it can not be divided into count and cube".to_string(),
                s.to_string(),
            ))
        }
    }
}

impl Pull {
    fn new(cube: Cube, count: u32) -> Self {
        Self { cube, count }
    }
}

struct Turn {
    pulls: Vec<Pull>,
}

impl FromStr for Turn {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pulls = s
            .split(',')
            .map(|ps| ps.trim())
            .map(|p| p.parse::<Pull>())
            .collect::<Result<Vec<_>, Self::Err>>()?;

        if pulls.is_empty() {
            Err(ParseError::new(
                "Could not split turn into pulls".to_string(),
                s.to_string(),
            ))
        } else {
            Ok(Turn::new(pulls))
        }
    }
}

impl Turn {
    fn new(pulls: Vec<Pull>) -> Self {
        Self { pulls }
    }
}

struct Rule {
    for_cube: Cube,
    max_count: u32,
}

impl Rule {
    fn new(for_cube: Cube, max_count: u32) -> Self {
        Self {
            for_cube,
            max_count,
        }
    }
}

struct Ruleset {
    rules: HashMap<Cube, u32>,
}

impl Ruleset {
    fn new(rules: Vec<Rule>) -> Self {
        Self {
            rules: rules
                .iter()
                .map(|r| (r.for_cube, r.max_count))
                .collect::<HashMap<_, _>>(),
        }
    }

    fn rule_for(&self, cube: &Cube) -> u32 {
        self.rules
            .get(cube)
            .cloned()
            .expect("to have rules for all cubes")
    }

    fn power(&self) -> u32 {
        self.rules.values().fold(1, |acc, e| acc * e)
    }
}
fn get_games() -> Vec<Game> {
    INPUT
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Game>())
        .collect::<Result<Vec<_>, ParseError>>()
        .expect("to be able to parse list of games")
}

fn part1() {
    let rules = vec![
        Rule::new(Cube::Red, 12),
        Rule::new(Cube::Green, 13),
        Rule::new(Cube::Blue, 14),
    ];
    let ruleset = Ruleset::new(rules);
    let games_played: Vec<Game> = get_games();

    let sum = games_played
        .iter()
        .filter(|g| g.is_valid_for_rules(&ruleset))
        .map(|vg| vg.id)
        .sum::<u32>();

    println!("Day-02 Part01: Sum of valid game id's is: {sum}");
}

fn part2() {
    let games_played: Vec<Game> = get_games();

    let power_sum = games_played
        .into_iter()
        .map(|g| g.game_ruleset())
        .map(|rs| rs.power())
        .sum::<u32>();

    println!("Day-02 Part02: Sum of power of rulesets is: {power_sum}");
}
