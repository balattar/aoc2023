/*
--- Day 2: Cube Conundrum ---

You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky.
You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time.
They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue.
Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag.
He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input).
Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

In game 1, three sets of cubes are revealed from the bag (and then put back again).
The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration.
However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once.
If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
*/

use itertools::Itertools;
use regex::Regex;

static INPUT: &str = std::include_str!("../input.txt");

fn get_input_vec(raw_input: &str) -> Vec<&str> {
    raw_input.split("\n").collect()
}

fn get_num_from_round_regex(regex: &Regex, round: &str) -> u32 {
    match regex.captures(round) {
        Some(captures) => match captures.get(1) {
            Some(m) => m.as_str().parse::<u32>().unwrap(),
            None => 0,
        },
        None => 0,
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (gamestr, roundstr): (&str, &str) = value.split(":").collect_tuple().unwrap();

        let id_regex = Regex::new("Game ([0-9]+)").unwrap();
        let id = get_num_from_round_regex(&id_regex, gamestr);

        let red_regex = Regex::new("([0-9]+) red").unwrap();
        let green_regex = Regex::new("([0-9]+) green").unwrap();
        let blue_regex = Regex::new("([0-9]+) blue").unwrap();

        let rounds: Vec<Round> = roundstr
            .split(";")
            .collect::<Vec<&str>>()
            .iter()
            .map(|round| {
                let red = get_num_from_round_regex(&red_regex, round);
                let green = get_num_from_round_regex(&green_regex, round);
                let blue = get_num_from_round_regex(&blue_regex, round);
                Round { red, green, blue }
            })
            .collect();

        Self { id, rounds }
    }
}
#[derive(Debug, Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn part1(input_list: &Vec<&str>) -> u32 {
    let games: Vec<Game> = input_list.iter().map(|&l| (*l).into()).collect();

    let valid_games: Vec<&Game> = games
        .iter()
        .filter(|&game| {
            !game
                .rounds
                .iter()
                .any(|round| round.red > 12 || round.green > 13 || round.blue > 14)
        })
        .collect();

    let valid_id_sum = valid_games.iter().fold(0, |acc, x| acc + x.id);

    valid_id_sum
}

fn part2(input_list: &Vec<&str>) -> u32 {
    let games: Vec<Game> = input_list.iter().map(|&l| (*l).into()).collect();

    let min_colors: Vec<Round> = games
        .iter()
        .map(|game| {
            let red = game
                .rounds
                .iter()
                .max_by(|&x, &y| x.red.cmp(&y.red))
                .unwrap()
                .red;
            let green = game
                .rounds
                .iter()
                .max_by(|&x, &y| x.green.cmp(&y.green))
                .unwrap()
                .green;
            let blue = game
                .rounds
                .iter()
                .max_by(|&x, &y| x.blue.cmp(&y.blue))
                .unwrap()
                .blue;
            Round { red, green, blue }
        })
        .collect();

    let power_sum: u32 = min_colors.iter().map(|mc| mc.power()).sum();

    power_sum
}
fn main() {
    let input_list = get_input_vec(INPUT);

    let part1_answer = part1(&input_list);
    dbg!(part1_answer);

    let part2_answer = part2(&input_list);
    dbg!(part2_answer);
}
