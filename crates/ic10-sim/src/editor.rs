use ratatui::crossterm::event::Event;
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::{Block, BorderType, Widget};

use crate::actions::EditorAction;
use crate::app::AppState;

/* ---------- */

/// Editor widget
#[derive(Debug, Default)]
pub(crate) struct Editor {
    /// Is the editor active?
    active: bool,

    /// Editor mode.
    mode: Mode,
}

impl Editor {
    /// Returns a new default [`Editor`] widget.
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Handle events when the editor is active.
    pub(crate) fn handle_event(&mut self, app_state: &mut AppState, event: Event) {
        let Some(action) = EditorAction::from_event(event) else {
            return;
        };

        match action {
            EditorAction::Quit => {
                app_state.quit();
                self.active = false;
            }
        }
    }

    /// Activate the editor.
    pub(crate) fn activate(&mut self) {
        self.active = true;
    }
}

impl Widget for &Editor {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let border = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(" Editor ");
        let border = if self.active {
            border.title_bottom(self.mode.as_border_title())
        } else {
            border
        };

        border.render(area, buf);
    }
}

/* ---------- */

/// Editor mode.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Mode {
    /// Characters typed in this mode will be inserted at the cursor position.
    #[default]
    Insertion,
    /// Character typed in this mode will replace characters at the cursor position
    /// except if the cursor is at the end of a line, in which case, the character will be appended to said line.
    Replacement,
}

impl Mode {
    /// Returns the string representation to be use as a border title.
    #[inline(always)]
    const fn as_border_title(self) -> &'static str {
        match self {
            Self::Insertion => " Insertion ",
            Self::Replacement => " Replacement ",
        }
    }
}
