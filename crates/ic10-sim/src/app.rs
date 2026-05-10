use ratatui::DefaultTerminal;
use ratatui::crossterm;
use ratatui::crossterm::event::Event;
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::Widget;

use std::io::Result as IoResult;
use std::time::Duration;

use crate::actions::GlobalAction;

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

/// App's main loop.
pub(crate) fn run(terminal: &mut DefaultTerminal) -> IoResult<()> {
    let app = App::new();

    loop {
        terminal.draw(|frame| frame.render_widget(&app, frame.area()))?;
        let Some(event) = get_event(Duration::from_micros(16700))? else {
            continue;
        };

        if let Some(GlobalAction::Quit) = GlobalAction::from_event(event) {
            break;
        }
    }

    Ok(())
}

/* ---------- */

/// Returns an IO event.
///
/// If the polling times out, the function returns `None`.
///
/// # Errors
///
/// Returns an error if something when wrong when polling or reading an event.
pub(crate) fn get_event(timeout: Duration) -> IoResult<Option<Event>> {
    if !crossterm::event::poll(timeout)? {
        return Ok(None);
    }

    Ok(Some(crossterm::event::read()?))
}
