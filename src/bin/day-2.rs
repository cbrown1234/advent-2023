use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};
use itertools::Itertools;
use thiserror::Error;

// require max: 12 red cubes, 13 green cubes, and 14 blue cubes per game
const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

#[derive(Error, Debug)]
pub enum AssumptionError {
    #[error("invalid header (expected '#=red', '#=green' or '#=blue', found {colour:?})")]
    InvalidColour { colour: String },
}

#[derive(Default, Debug)]
struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_game(desc: &str) -> Result<Game> {
    let mut game = Game::default();
    dbg!(desc);
    let colour_descs = desc.split(',').map(|num_colour| num_colour.trim());

    for num_colour in colour_descs {
        match num_colour.split_once(' ') {
            Some((reds, "red")) => {
                game.red = reds.parse::<i32>().context("failed to parse red number")?
            }
            Some((greens, "green")) => {
                game.green = greens
                    .parse::<i32>()
                    .context("failed to parse green number")?
            }
            Some((blues, "blue")) => {
                game.blue = blues
                    .parse::<i32>()
                    .context("failed to parse blue number")?
            }
            _ => Err(AssumptionError::InvalidColour {
                colour: num_colour.to_owned(),
            })?,
        }
    }
    Ok(game)
}

fn parse_games_desc(desc: &str) -> Result<Vec<Game>> {
    desc.split(';').map(|game| parse_game(game)).collect()
}

fn main() -> Result<()> {
    let file = File::open("inputs/day2.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader
        .lines()
        .map(|line| line.expect("able to read file"));

    let game_id_sum = lines
        .map(|line| {
            eprintln!("{line}");
            let (id, games) = line
                .strip_prefix("Game ")
                .context("has no 'Game ' prefix")?
                .split_once(':')
                .context("has no ':' separator")?;
            Ok((
                id.parse::<i32>().context("game id not valid int")?,
                parse_games_desc(&games)?,
            ))
        })
        .map_ok(|(id, games)| {
            let result = (
                id,
                games.iter().fold(Game::default(), |max_seen, x| {
                    dbg!(x);
                    Game {
                        red: max_seen.red.max(x.red),
                        green: max_seen.green.max(x.green),
                        blue: max_seen.blue.max(x.blue),
                    }
                }),
            );
            dbg!(&result);
            result
        })
        .filter_ok(|(_id, game)| {
            game.red <= MAX_RED && game.green <= MAX_GREEN && game.blue <= MAX_BLUE
        })
        .map_ok(|(id, _)| id)
        .sum::<Result<i32>>()?;

    println!("Valid games sum = {game_id_sum}");

    let file = File::open("inputs/day2.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader
        .lines()
        .map(|line| line.expect("able to read file"));

    let game_id_sum = lines
        .map(|line| {
            eprintln!("{line}");
            let (id, games) = line
                .strip_prefix("Game ")
                .context("has no 'Game ' prefix")?
                .split_once(':')
                .context("has no ':' separator")?;
            Ok((
                id.parse::<i32>().context("game id not valid int")?,
                parse_games_desc(&games)?,
            ))
        })
        .map_ok(|(id, games)| {
            let result = (
                id,
                games.iter().fold(Game::default(), |max_seen, x| {
                    dbg!(x);
                    Game {
                        red: max_seen.red.max(x.red),
                        green: max_seen.green.max(x.green),
                        blue: max_seen.blue.max(x.blue),
                    }
                }),
            );
            dbg!(&result);
            result
        })
        .map_ok(|(_id, game)| game.red * game.green * game.blue)
        .sum::<Result<i32>>()?;
    let game_id_sum = game_id_sum;

    println!("Colour power sum = {game_id_sum}");
    Ok(())
}
