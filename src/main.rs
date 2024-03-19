use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};

mod app;
mod errors;
mod tui;
mod ui;

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    let app = app::App::new();
    loop {
        terminal.draw(|f| ui::render_ui(f, &app));
        if should_quit()? {
            break;
        }
    }

    tui::restore()?;
    Ok(())
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
