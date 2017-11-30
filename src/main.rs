use std::io::{self, BufReader, BufRead, BufWriter};

mod bot;
mod field;
mod player;
mod error;

use error::*;

enum Message {
    Settings(Setting),
    Update(Update),
    Action(Action),
}

enum Setting {
    TimeBank(i32),
    TimePerMove(i32),
    PlayerNames(Vec<String>),
    YourBot(String),
    YourBotId(i32),
    FieldWidth(i32),
    FieldHeight(i32),
    MaxRounds(i32),
}

enum Update {
    GameRound(i32),
    GameField(Vec<Cell>),
    PlayerSnippets(i32),
    PlayerBombs(i32),
}

enum Action {
    Character { time_to_respond: i32 },
    Move { time_to_respond: i32 },
}

pub struct Cell {
    cell_items: Vec<CellItem>,
}

pub enum CellItem {
    Empty,
    Inaccessible,
    Player(i32),
    Spawn(Option<i32>),
    Gate(GateDirection),
    Enemy(i32),
    Bomb(i32),
    CodeSnippet,
}

pub enum GateDirection {
    Left,
    Right,
}

fn main() {
    let status = match start() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    };

    std::process::exit(status);
}

fn start() -> Result<()> {
    let output = BufWriter::new(io::stdout());
    let err = BufWriter::new(io::stderr());

    for line in BufReader::new(io::stdin())
        .lines()
        .map(|r| r.map_err(|e| Error::IoError(e)))
        .filter_map(Result::ok) {
        let msg = line.parse::<Message>()?;
    }
    Ok(())
}

impl std::str::FromStr for Message {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use error::ParseErrorKind::{Incomplete, InvalidOption, NumberFormat, UnknownCommand};
        let mut words = s.split_whitespace();
        let command = (words.next().ok_or(Error::ParseError(Incomplete))?,
                       words.next().ok_or(Error::ParseError(Incomplete))?,
                       words.next().ok_or(Error::ParseError(Incomplete))?,
                       words.next());

        match command {
            ("settings", "timebank", value, None) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Settings(Setting::TimeBank(n))))
                    .map_err(|_| Error::ParseError(NumberFormat))
            }
            ("settings", "time_per_move", value, None) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Settings(Setting::TimePerMove(n))))
                    .map_err(|_| Error::ParseError(NumberFormat))
            }
            ("settings", "player_names", value, None) => {
                let names = value.split(",").map(|s| s.into()).collect();
                Ok(Message::Settings(Setting::PlayerNames(names)))
            }
            ("settings", "your_bot", value, None) => {
                Ok(Message::Settings(Setting::YourBot(value.into())))
            }
            ("settings", "your_botid", value, None) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Settings(Setting::YourBotId(n))))
                    .map_err(|e| Error::ParseError(NumberFormat))
            }
            ("settings", "field_width", value, None) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Settings(Setting::FieldWidth(n))))
                    .map_err(|e| Error::ParseError(NumberFormat))
            }
            ("settings", "field_height", value, None) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Settings(Setting::FieldHeight(n))))
                    .map_err(|e| Error::ParseError(NumberFormat))
            }
            ("settings", "max_rounds", value, None) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Settings(Setting::MaxRounds(n))))
                    .map_err(|e| Error::ParseError(NumberFormat))
            }
            ("update", "game", "round", Some(value)) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Update(Update::GameRound(n))))
                    .map_err(|e| Error::ParseError(NumberFormat))
            }
            ("update", "game", "field", Some(value)) => {
                // TODO parse field into vector
                Ok(Message::Update(Update::GameField(Vec::new())))
            }
            ("update", player, "bombs", Some(value)) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Update(Update::PlayerBombs(n))))
                    .map_err(|e| Error::ParseError(NumberFormat))
            }
            ("update", player, "snippets", Some(value)) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Update(Update::PlayerSnippets(n))))
                    .map_err(|e| Error::ParseError(NumberFormat))
            }
            ("action", "character", time, None) => {
                time.parse::<i32>()
                    .and_then(|n| Ok(Message::Action(Action::Character { time_to_respond: n })))
                    .map_err(|e| Error::ParseError(NumberFormat))
            }
            ("action", "move", time, None) => {
                time.parse::<i32>()
                    .and_then(|n| Ok(Message::Action(Action::Move { time_to_respond: n })))
                    .map_err(|e| Error::ParseError(NumberFormat))
            }
            (_, _, _, _) => Err(Error::ParseError(UnknownCommand)),
        }
    }
}
