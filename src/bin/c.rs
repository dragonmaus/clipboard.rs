extern crate cli_clipboard;

use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self, Read};

program::main!("c");

fn program(_name: &str) -> program::Result {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut provider: ClipboardContext = ClipboardProvider::new()?;
    provider.set_contents(input)?;

    Ok(0)
}
