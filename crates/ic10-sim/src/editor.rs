use ic10_sim_program::Program;
use ratatui::crossterm::event::Event;
use ratatui::layout::Position;
use ratatui::prelude::{Buffer, Rect};
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};

use crate::actions::EditorAction;
use crate::app::AppState;

/* ---------- */

/// Editor widget
#[derive(Debug, Default)]
pub(crate) struct Editor {
    active: bool,
    mode: Mode,

    program: Program,
    cursor: Cursor,
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
                self.cursor.toggle();
            }
            EditorAction::ToggleMode => {
                self.mode.toggle();
            }
            EditorAction::AddCharacter(c) => {
                if self
                    .program
                    .insert_char(self.cursor.line(), self.cursor.col(), c)
                {
                    self.cursor.move_right();
                }
            }
            EditorAction::RemoveCharacter => {
                if self
                    .program
                    .remove_char(self.cursor.line(), self.cursor.col())
                {
                    self.cursor.move_left();
                }
            }
            EditorAction::InsertNewLine => {
                if self
                    .program
                    .insert_new_line_at(self.cursor.line(), self.cursor.col())
                {
                    self.cursor.set_position(self.cursor.line() + 1, 0);
                }
            }
        }
    }

    /// Activate the editor.
    #[inline(always)]
    pub(crate) fn activate(&mut self) {
        self.active = true;
        self.cursor.toggle();
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
        let inner_area = border.inner(area);

        let text = Text::from_iter(self.program.lines().map(|line| line.as_str()));
        Paragraph::new(text).render(inner_area, buf);

        self.cursor.render(inner_area, buf);
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

/// Editor cursor.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Cursor {
    line: u16,
    col: u16,
    visible: bool,
}

impl Cursor {
    /// Returns the line position of the cursor.
    #[inline(always)]
    fn line(&self) -> usize {
        self.line as usize
    }

    /// Returns the column position of the cursor.
    #[inline(always)]
    fn col(&self) -> usize {
        self.col as usize
    }

    /// Toggle the cursor's visibility.
    #[inline(always)]
    fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Sets the cursor position.
    #[inline(always)]
    fn set_position(&mut self, line: usize, col: usize) {
        self.line = line.min(u16::MAX as usize) as u16;
        self.col = col.min(u16::MAX as usize) as u16;
    }

    /// Moves the cursor one cell to the right.
    #[inline(always)]
    fn move_right(&mut self) {
        self.col = self.col.saturating_add(1)
    }

    /// Moves the cursor one cell to the right.
    #[inline(always)]
    fn move_left(&mut self) {
        self.col = self.col.saturating_sub(1)
    }
}

impl Widget for &Cursor {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if !self.visible {
            return;
        }

        let abs_position = Position::new(self.col + area.x, self.line + area.y);
        let Some(cell) = buf.cell_mut(abs_position) else {
            return;
        };

        cell.set_style(Style::new().reversed());
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

    #[cfg(test)]
    mod cursor {
        use super::*;

        #[test]
        fn move_left() {
            let truth = Cursor {
                line: 0,
                col: 0,
                visible: false,
            };
            let mut cursor = Cursor {
                line: 0,
                col: 1,
                visible: false,
            };

            cursor.move_left();
            assert_eq!(cursor, truth);
        }

        #[test]
        fn move_right() {
            let truth = Cursor {
                line: 0,
                col: 1,
                visible: false,
            };
            let mut cursor = Cursor {
                line: 0,
                col: 0,
                visible: false,
            };

            cursor.move_right();
            assert_eq!(cursor, truth);
        }
    }
}
