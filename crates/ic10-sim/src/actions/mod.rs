use ratatui::crossterm::event::Event;

mod editor;
mod global;
mod idle;

pub(crate) use editor::EditorAction;
pub(crate) use global::GlobalAction;
pub(crate) use idle::IdleAction;

/* ---------- */

pub(crate) trait FromEvent: Sized {
    fn from_event(event: &Event) -> Option<Self>;
}
