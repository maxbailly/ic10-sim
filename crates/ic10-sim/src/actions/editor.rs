use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};

/* ---------- */

/// Actions that must be handled when the app is idling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum EditorAction {
    /// Quit the TUI application.
    Quit,
}

impl EditorAction {
    /// Consumes and converts an [`Event`] to a [`IdleAction`] if the given event matches.
    ///
    /// If the event doesn't match anything, returns `None`.
    pub(crate) fn from_event(event: Event) -> Option<Self> {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => Some(Self::Quit),
            // Unmatching inputs
            _ => None,
        }
    }
}
