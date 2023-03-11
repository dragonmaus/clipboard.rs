use cli_clipboard::{ClipboardContext, ClipboardProvider};

program::main!("n");

fn program(_name: &str) -> program::Result {
    let mut provider: ClipboardContext = ClipboardProvider::new()?;
    provider.set_contents(String::new())?;

    Ok(0)
}
