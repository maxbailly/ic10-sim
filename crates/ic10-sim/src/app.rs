use ratatui::DefaultTerminal;
use ratatui::crossterm;
use ratatui::crossterm::event::Event;
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::Widget;

use std::io::Result as IoResult;
use std::time::Duration;

use crate::actions::GlobalAction;
use crate::actions::IdleAction;

/* ---------- */

/// Main UI of the app.
pub(crate) struct App {
    running: bool,
}

impl App {
    /// Creates a new default [`App`].
    fn new() -> Self {
        Self { running: true }
    }

    /// Updates the UI according the given input.
    fn update(&mut self, event: Event) {
        let event = IdleAction::from_event(&event);

        // TODO: there's no states rn but once there is, change that to a match
        if let Some(IdleAction::Quit) = event {
            self.running = false
        }
    }

    /// Returns `true` if the TUI app is running, `false` otherwise.
    #[inline(always)]
    fn is_running(&self) -> bool {
        self.running
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
    let mut app = App::new();

    while app.is_running() {
        terminal.draw(|frame| frame.render_widget(&app, frame.area()))?;
        let Some(event) = get_event(Duration::from_micros(16700))? else {
            continue;
        };

        if let Some(GlobalAction::Quit) = GlobalAction::from_event(&event) {
            break;
        }

        app.update(event);
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
#[inline(always)]
fn get_event(timeout: Duration) -> IoResult<Option<Event>> {
    if !crossterm::event::poll(timeout)? {
        return Ok(None);
    }

    Ok(Some(crossterm::event::read()?))
}
