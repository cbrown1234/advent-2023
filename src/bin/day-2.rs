use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

// require max: 12 red cubes, 13 green cubes, and 14 blue cubes per game
const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

#[derive(Default, Debug)]
struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_game(desc: &str) -> Game {
    let mut game = Game::default();
    dbg!(desc);
    desc.split(',')
        .map(|num_colour| num_colour.trim())
        .map(|num_colour| match num_colour.split_once(' ') {
            Some((reds, "red")) => game.red = reds.parse::<i32>().expect("Valid number of colour"),
            Some((greens, "green")) => {
                game.green = greens.parse::<i32>().expect("Valid number of colour");
            }
            Some((blues, "blue")) => {
                game.blue = blues.parse::<i32>().expect("Valid number of colour");
            }
            _ => {}
        })
        .count();
    game
}

fn parse_games_desc(desc: &str) -> Vec<Game> {
    desc.split(';').map(|game| parse_game(game)).collect()
}

fn main() -> Result<()> {
    let file = File::open("inputs/day2.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader
        .lines()
        .map(|line| line.expect("able to read file"));

    let game_id_sum: i32 = lines
        .map(|line| {
            eprintln!("{line}");
            let line = line
                .strip_prefix("Game ")
                .expect("has 'Game ' prefix")
                .split_once(':')
                .expect("has ':' separator");
            (line.0.to_owned(), line.1.to_owned())
        })
        .map(|(id, games)| {
            (
                id.parse::<i32>().expect("game id to be valid int"),
                parse_games_desc(&games),
            )
        })
        .map(|(id, games)| {
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
        .filter(|(_id, game)| {
            game.red <= MAX_RED && game.green <= MAX_GREEN && game.blue <= MAX_BLUE
        })
        .map(|(id, _)| id)
        .sum();

    println!("Valid games sum = {game_id_sum}");


    let file = File::open("inputs/day2.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader
        .lines()
        .map(|line| line.expect("able to read file"));

    let game_id_sum: i32 = lines
        .map(|line| {
            eprintln!("{line}");
            let line = line
                .strip_prefix("Game ")
                .expect("has 'Game ' prefix")
                .split_once(':')
                .expect("has ':' separator");
            (line.0.to_owned(), line.1.to_owned())
        })
        .map(|(id, games)| {
            (
                id.parse::<i32>().expect("game id to be valid int"),
                parse_games_desc(&games),
            )
        })
        .map(|(id, games)| {
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
        .map(|(_id, game)| {
            game.red * game.green * game.blue
        })
        .sum();

    println!("Colour power sum = {game_id_sum}");
    Ok(())
}
