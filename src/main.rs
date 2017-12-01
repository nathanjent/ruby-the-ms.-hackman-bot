extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_file_unix;

use futures::Stream;
use futures::sync::mpsc;
use tokio_io::io;
use tokio_file_unix::{File, StdFile, DelimCodec, Newline};
use std::io::{self as stdio, Write};
//use std::io::{self, BufReader, BufRead, BufWriter, Write};
//use std::rc::Rc;
//use std::cell::RefCell;

mod bot;
mod field;
mod player;
mod error;
mod message;

use error::*;
use bot::BotState;
use player::Player;
use message::*;

fn main() {
    let status = match start() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    };
    ::std::process::exit(status);
}

fn start() -> Result<()> {
    // initialize the event loop
    let mut core = tokio_core::reactor::Core::new()?;
    let handle = core.handle();

    // get the standard io as a file
    let stdin = stdio::stdin();
    let stdout = stdio::stdout();
    let stderr = stdio::stderr();
    let reader = File::new_nb(StdFile(stdin.lock()))?.into_reader(&handle)?;
    let mut writer = File::new_nb(StdFile(stdout.lock()))?.into_io(&handle)?;
    let mut err = File::new_nb(StdFile(stderr.lock()))?.into_io(&handle)?;

    let bot = BotState::new();

    // turn it into a stream of lines and process them
    let future = io::lines(reader).for_each(|line| {
        match handle_message(line, &bot) {
            Ok(output) => {
                match output {
                    Some(o) => writeln!(writer, "{}", o),
                    None => Ok(())
                }
            }
            Err(e) => writeln!(err, "Error: {}", e),
        };

        Ok(())
    });

    // start the event loop
    core.run(future)?;
    Ok(())
}

fn handle_message(line: String, bot: &BotState) -> Result<Option<String>> {
    let msg = line.parse::<Message>()?;
    let reply = match msg {
        Message::Settings(Setting::TimeBank(n)) => {
            bot.settings.borrow_mut().time_bank = n;
            Some(format!("set time_bank {}", n))
        }
        Message::Settings(Setting::TimePerMove(n)) => {
            bot.settings.borrow_mut().time_per_move = n;
            Some(format!("set time_per_move {}", n))
        }
        Message::Settings(Setting::PlayerNames(names)) => {
            for name in names {
                let player_name = name.clone();
                bot.settings
                    .borrow_mut()
                    .players.insert(name, Player::new(player_name));
            }
            None
        }
        Message::Settings(Setting::YourBot(bot_name)) => {
            bot.settings.borrow_mut().name = bot_name;
            None
        }
        Message::Settings(Setting::YourBotId(id)) => {
            bot.settings.borrow_mut().id = id;
            None
        }
        Message::Settings(Setting::FieldWidth(w)) => {
            bot.settings.borrow_mut().field.field.width = w as usize;
            None
        }
        Message::Settings(Setting::FieldHeight(h)) => {
            bot.settings.borrow_mut().field.field.height = h as usize;
            None
        }
        Message::Settings(Setting::MaxRounds(max)) => {
            bot.settings.borrow_mut().max_rounds = max;
            None
        }
        Message::Update(Update::GameRound(n)) => {
            bot.settings.borrow_mut().round = n;
            None
        }
        Message::Update(Update::GameField(field)) => {
            Some("not implemented".into())
        }
        Message::Action(Action::Character { time_to_respond: n }) => {
            Some("bixie".into())
        }
        Message::Action(Action::Move { time_to_respond: n }) => {
            if let Some(detonation_time) = bot.player.borrow().bomb_drop {
                Some(format!("up;drop_bomb {}", detonation_time))
            } else {
                Some("up".into())
            }
        }
        _ => return Err(Error::UnknownCommand),
    };
    Ok(reply)
}

//fn start() -> Result<()> {
//    let stdout = io::stdout();
//    let stderr = io::stderr();
//    let stdin = io::stdin();
//    let bot = BotState::new();
//
//    for line in stdin.lock()
//        .lines()
//        .map(|r| r.map_err(|e| Error::IoError(e)))
//        .filter_map(Result::ok) {
//            match handle_message(line, &bot) {
//                Ok(output) => {
//                    match output {
//                        Some(o) => writeln!(stdout.lock(), "{}", o),
//                        None => Ok(())
//                    }
//                }
//                Err(e) => writeln!(stderr.lock(), "Error: {}", e),
//            };
//    }
//    Err(Error::UnintentionalBreak)
//}
