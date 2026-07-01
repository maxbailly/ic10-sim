use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};

use crate::actions::FromEvent;

/* ---------- */

/// Actions that must be handled when the app is idling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum EditorAction {
    /// Quit the TUI application.
    Quit,
    /// Toggle the edition mode.
    ToggleMode,
    /// Add the character to the program.
    AddCharacter(char),
    /// Removes the previous character from the program.
    RemovePrevCharacter,
    /// Insert a new line in the program.
    InsertNewLine,
    /// Move cursor one cell to the left.
    MoveCursorLeft,
    /// Move cursor one cell to the right.
    MoveCursorRight,
    /// Move cursor one cell up.
    MoveCursorUp,
    /// Move cursor one cell down.
    MoveCursorDown,
}

impl FromEvent for EditorAction {
    /// Consumes and converts an [`Event`] to a [`IdleAction`] if the given event matches.
    ///
    /// If the event doesn't match anything, returns `None`.
    fn from_event(event: &Event) -> Option<Self> {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => Some(Self::Quit),
            Event::Key(KeyEvent {
                code: KeyCode::Insert,
                ..
            }) => Some(Self::ToggleMode),
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => Some(Self::AddCharacter(*c)),
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => Some(Self::RemovePrevCharacter),
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => Some(Self::InsertNewLine),
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => Some(Self::MoveCursorLeft),
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => Some(Self::MoveCursorRight),
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => Some(Self::MoveCursorUp),
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => Some(Self::MoveCursorDown),
            // Unmatching inputs
            _ => None,
        }
    }
}
