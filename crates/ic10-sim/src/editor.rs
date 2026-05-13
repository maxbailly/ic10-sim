use ic10_sim_program::Program;
use ratatui::crossterm::event::Event;
use ratatui::prelude::{Buffer, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};

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

    /// IC10 program.
    program: Program,
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
            EditorAction::ToggleMode => {
                self.mode.toggle();
            }
            EditorAction::AddCharacter(c) => {
                self.program.insert_char(0, 0, c);
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

        let text = Text::from_iter(self.program.lines().map(|line| line.as_str()));
        Paragraph::new(text).render(border.inner(area), buf);

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
    /// Toggles the edition mode.
    #[inline(always)]
    fn toggle(&mut self) {
        *self = match *self {
            Self::Insertion => Self::Replacement,
            Self::Replacement => Self::Insertion,
        };
    }

    /// Returns the string representation to be use as a border title.
    #[inline(always)]
    const fn as_border_title(self) -> &'static str {
        match self {
            Self::Insertion => " Insertion ",
            Self::Replacement => " Replacement ",
        }
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod mode {
        use super::*;

        #[test]
        fn default() {
            assert_eq!(Mode::default(), Mode::Insertion)
        }

        #[test]
        fn toggle() {
            let mut mode = Mode::default();

            mode.toggle();
            assert_eq!(mode, Mode::Replacement);

            mode.toggle();
            assert_eq!(mode, Mode::Insertion);
        }
    }
}
