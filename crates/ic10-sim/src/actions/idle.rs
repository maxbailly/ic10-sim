use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};

use super::FromEvent;

/* ---------- */

/// Actions that must be handled when the app is idling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum IdleAction {
    /// Quit the TUI application.
    Quit,
    /// Start editing the program.
    StartEdition,
}

impl FromEvent for IdleAction {
    /// Consumes and converts an [`Event`] to a [`IdleAction`] if the given event matches.
    ///
    /// If the event doesn't match anything, returns `None`.
    fn from_event(event: &Event) -> Option<Self> {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => Some(Self::Quit),
            Event::Key(KeyEvent {
                code: KeyCode::Insert,
                ..
            }) => Some(Self::StartEdition),
            // Unmatching inputs
            _ => None,
        }
    }
}
