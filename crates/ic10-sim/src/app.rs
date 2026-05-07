use ratatui::DefaultTerminal;
use ratatui::crossterm;
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::Widget;

use std::io::Result as IoResult;

/* ---------- */

/// Main UI of the app.
pub(crate) struct App;

impl App {
    /// Creates a new default [`App`].
    fn new() -> Self {
        Self
    }
}

impl Widget for &App {
    fn render(self, _area: Rect, _buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}

/* ---------- */

pub(crate) fn run(terminal: &mut DefaultTerminal) -> IoResult<()> {
    let app = App::new();

    loop {
        terminal.draw(|frame| frame.render_widget(&app, frame.area()))?;
        if crossterm::event::read()?.is_key_press() {
            break;
        }
    }

    Ok(())
}
