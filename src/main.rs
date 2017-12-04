extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_file_unix;

use futures::Stream;
use tokio_io::io;
use tokio_file_unix::{File, StdFile};
use std::io::{self as stdio, Write};

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
