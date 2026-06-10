use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::Size,
};

/* ---------- */

/// Actions that must be handled from whatever state the UI is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum GlobalAction {
    /// Quit the TUI application.
    Quit,
    /// The terminal window has been resized
    Resize(Size),
}

impl GlobalAction {
    /// Converts an [`Event`] to a [`GlobalAction`] if the given event matches.
    ///
    /// If the event doesn't match anything, returns `None`.
    pub(crate) fn from_event(event: &Event) -> Option<Self> {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => Some(Self::Quit),
            Event::Resize(column, row) => Some(Self::Resize(Size::new(*column, *row))),
            // Unmatching inputs
            _ => None,
        }
    }
}
