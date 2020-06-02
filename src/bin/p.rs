use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self, Write};

program::main!("p");

fn program() -> program::Result {
    let mut provider: ClipboardContext = ClipboardProvider::new()?;
    io::stdout().write_all(&provider.get_contents()?.into_bytes())?;

    Ok(0)
}
