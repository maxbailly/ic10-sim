use ratatui::DefaultTerminal;
use ratatui::crossterm;
use ratatui::crossterm::event::Event;
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::Widget;

use std::io::Result as IoResult;
use std::time::Duration;

use crate::actions::GlobalAction;
use crate::actions::IdleAction;
use crate::editor::Editor;

/* ---------- */

/// Main UI of the app.
pub(crate) struct App {
    /// Is the app currently running?
    running: bool,
    /// App state.
    state: AppState,

    /// Editor widget.
    editor: Editor,
}

impl App {
    /// Creates a new default [`App`].
    #[inline(always)]
    fn new() -> Self {
        Self {
            running: true,
            state: AppState::default(),
            editor: Editor::new(),
        }
    }

    /// Updates the UI according the given input.
    #[inline(always)]
    fn update(&mut self, event: Event) {
        match self.state {
            AppState::Idle => self.handle_event(event),
            AppState::Editing => self.editor.handle_event(&mut self.state, event),
        }
    }

    /// Handles event happening when the app is idling.
    fn handle_event(&mut self, event: Event) {
        let action = IdleAction::from_event(event);

        match action {
            Some(IdleAction::Quit) => self.running = false,
            Some(IdleAction::StartEdition) => {
                self.state = AppState::Editing;
                self.editor.activate();
            }
            None => (),
        }
    }

    /// Returns `true` if the TUI app is running, `false` otherwise.
    #[inline(always)]
    fn is_running(&self) -> bool {
        self.running
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.editor.render(area, buf);
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

/// App state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AppState {
    /// The app is idling.
    #[default]
    Idle,
    /// The editor is active.
    Editing,
}

impl AppState {
    pub(crate) fn quit(&mut self) {
        if let Self::Editing = *self {
            *self = Self::Idle
        }
    }
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
