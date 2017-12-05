#[cfg(external)] extern crate futures;
#[cfg(external)] extern crate tokio_core;
#[cfg(external)] extern crate tokio_io;
#[cfg(external)] extern crate tokio_file_unix;
 
#[cfg(external)] use futures::Stream;
#[cfg(external)] use tokio_io::io;
#[cfg(external)] use tokio_file_unix::{File, StdFile};
use std::io::{self as stdio, BufRead, Write};

mod bot;
mod error;
mod field;
mod handler;
mod message;
mod player;
mod simple_matrix;

use error::*;
use bot::BotState;
use handler::*;

#[cfg(not(test))]
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

/// Run the event loop
#[cfg(not(test))]
fn start() -> Result<()> {
    let stdin = stdio::stdin();
    let stdout = stdio::stdout();
    let stderr = stdio::stderr();
    let mut writer = stdout.lock();
    let mut err = stderr.lock();
    
    // initialize the game state
    let bot = BotState::new();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            match handle_message(line, &bot) {
                Ok(output) => {
                    match output {
                        Some(o) => writeln!(writer, "{}", o)?,
                        None => {},
                    }
                }
                Err(e) => writeln!(err, "Error: {}", e)?,
            }
        }
    }
    Err(Error::UnintentionalBreak)
}

// No external lib support.
#[cfg(external)]
#[cfg(not(test))]
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

    // initialize the game state
    let bot = BotState::new();

    // turn it into a stream of lines and process them
    let future = io::lines(reader).for_each(|line| {
        match handle_message(line, &bot) {
                Ok(output) => {
                    match output {
                        Some(o) => writeln!(writer, "{}", o),
                        None => Ok(()),
                    }
                }
                Err(e) => writeln!(err, "Error: {}", e),
            }
            .map(|_| ())
    });

    // start the event loop
    core.run(future)?;
    Ok(())
}
