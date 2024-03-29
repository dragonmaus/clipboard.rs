extern crate cli_clipboard;

use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self, Write};

program::main!("p");

fn program(_name: &str) -> program::Result {
    let mut provider: ClipboardContext = ClipboardProvider::new()?;
    io::stdout().write_all(
        &match provider.get_contents() {
            Err(e) if e.to_string() == "OS error 0: The operation completed successfully." => {
                Ok("".into())
            },
            result => result,
        }?
        .into_bytes(),
    )?;

    Ok(0)
}
