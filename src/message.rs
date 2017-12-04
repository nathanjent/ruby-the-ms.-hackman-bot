use error::*;
use field::{parse_field, Cell};

#[derive(Debug)]
pub enum Message {
    Settings(Setting),
    Update(Update),
    Action(Action),
}

#[derive(Debug)]
pub enum Setting {
    TimeBank(i32),
    TimePerMove(i32),
    PlayerNames(Vec<String>),
    YourBot(String),
    YourBotId(i32),
    FieldWidth(i32),
    FieldHeight(i32),
    MaxRounds(i32),
}

#[derive(Debug)]
pub enum Update {
    GameRound(i32),
    GameField(Vec<Cell>),
    PlayerSnippets(String, i32),
    PlayerBombs(String, i32),
}

#[derive(Debug)]
pub enum Action {
    Character { time_to_respond: i32 },
    Move { time_to_respond: i32 },
}

impl ::std::str::FromStr for Message {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use error::ParseErrorKind::{Incomplete, NumberFormat, UnknownCommand};
        let mut words = s.split_whitespace();
        let command = (words.next().ok_or(Error::ParseError(Incomplete))?,
                       words.next().ok_or(Error::ParseError(Incomplete))?,
                       words.next().ok_or(Error::ParseError(Incomplete))?,
                       words.next());

        match command {
            ("settings", "timebank", value, None) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Settings(Setting::TimeBank(n))))
                    .map_err(|e| Error::ParseError(NumberFormat(Box::new(e))))
            }
            ("settings", "time_per_move", value, None) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Settings(Setting::TimePerMove(n))))
                    .map_err(|e| Error::ParseError(NumberFormat(Box::new(e))))
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
                    .map_err(|e| Error::ParseError(NumberFormat(Box::new(e))))
            }
            ("settings", "field_width", value, None) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Settings(Setting::FieldWidth(n))))
                    .map_err(|e| Error::ParseError(NumberFormat(Box::new(e))))
            }
            ("settings", "field_height", value, None) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Settings(Setting::FieldHeight(n))))
                    .map_err(|e| Error::ParseError(NumberFormat(Box::new(e))))
            }
            ("settings", "max_rounds", value, None) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Settings(Setting::MaxRounds(n))))
                    .map_err(|e| Error::ParseError(NumberFormat(Box::new(e))))
            }
            ("update", "game", "round", Some(value)) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Update(Update::GameRound(n))))
                    .map_err(|e| Error::ParseError(NumberFormat(Box::new(e))))
            }
            ("update", "game", "field", Some(value)) => {
                // Parse field into vector
                parse_field(value)
                    .and_then(|field_cells| Ok(Message::Update(Update::GameField(field_cells))))
            }
            ("update", player, "bombs", Some(value)) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Update(Update::PlayerBombs(player.into(), n))))
                    .map_err(|e| Error::ParseError(NumberFormat(Box::new(e))))
            }
            ("update", player, "snippets", Some(value)) => {
                value.parse::<i32>()
                    .and_then(|n| Ok(Message::Update(Update::PlayerSnippets(player.into(), n))))
                    .map_err(|e| Error::ParseError(NumberFormat(Box::new(e))))
            }
            ("action", "character", time, None) => {
                time.parse::<i32>()
                    .and_then(|n| Ok(Message::Action(Action::Character { time_to_respond: n })))
                    .map_err(|e| Error::ParseError(NumberFormat(Box::new(e))))
            }
            ("action", "move", time, None) => {
                time.parse::<i32>()
                    .and_then(|n| Ok(Message::Action(Action::Move { time_to_respond: n })))
                    .map_err(|e| Error::ParseError(NumberFormat(Box::new(e))))
            }
            (_, _, _, _) => Err(Error::ParseError(UnknownCommand)),
        }
    }
}
