use std::error::Error;
use std::process;

use clap::Parser;

use blackbox::BlackBox;
use gnome_terminal::GnomeTerminal;

mod blackbox;
mod gnome_terminal;
mod themes;

trait Terminal {
    fn apply(theme: &themes::Theme) -> Result<(), Box<dyn Error>>;
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = themes::Args::parse();
    let theme = themes::get(&args)?;
    GnomeTerminal::apply(&theme)?;
    BlackBox::apply(&theme)?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
