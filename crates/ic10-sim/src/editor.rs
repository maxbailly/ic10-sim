use ic10_sim_program::Program;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::prelude::{Buffer, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};

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
                if self.cursor.col() == 0 {
                    if self.cursor.line() == 0 {
                        return;
                    }

                    let prev_line_len = self.program.line(self.cursor.line() - 1).len();
                    let merged = self.program.merge_with_previous_line(self.cursor.line());
                    if merged {
                        self.cursor
                            .set_position(self.cursor.line() - 1, prev_line_len);
                    }

                    return;
                }

                if self
                    .program
                    .remove_char(self.cursor.line(), self.cursor.col() - 1)
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
            EditorAction::MoveCursorLeft => {
                if self.cursor.col() != 0 {
                    self.cursor.move_left();
                    return;
                }

                if self.cursor.line() == 0 {
                    return;
                }

                let prev_line_idx = self.cursor.line() - 1;
                let prev_line_len = self.program.line(prev_line_idx).len();

                self.cursor.set_position(prev_line_idx, prev_line_len);
            }
            EditorAction::MoveCursorRight => {
                if self.program.is_empty() {
                    return;
                }

                let line_len = self.program.line(self.cursor.line()).len();
                if self.cursor.col() < line_len {
                    self.cursor.move_right();
                    return;
                }

                let nb_lines = self.program.iter().count();
                if self.cursor.line() >= nb_lines - 1 {
                    return;
                }

                let next_line_idx = self.cursor.line() + 1;
                self.cursor.set_position(next_line_idx, 0);
            }
            EditorAction::MoveCursorUp => {
                if self.cursor.line() == 0 || self.program.is_empty() {
                    return;
                }

                self.cursor.move_up();
                let line_len = self.program.line(self.cursor.line()).len();
                if self.cursor.col() > line_len {
                    self.cursor.set_col(line_len);
                }
            }
            EditorAction::MoveCursorDown => {
                if self.program.is_empty() || self.cursor.line() >= self.program.nb_lines() - 1 {
                    return;
                }

                self.cursor.move_down();
                let line_len = self.program.line(self.cursor.line()).len();
                if self.cursor.col() > line_len {
                    self.cursor.set_col(line_len);
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
        let inner_area = border.inner(area);

        let border = if self.active {
            let line_col_title = Line::from(format!(
                " L: {}, C: {} ",
                self.cursor.line(),
                self.cursor.col()
            ))
            .alignment(ratatui::layout::HorizontalAlignment::Right);

            border
                .title_bottom(self.mode.as_border_title())
                .title_bottom(line_col_title)
        } else {
            border
        };

        EditorBox::from_iter(self.program.iter().map(|line| line.as_str()))
            .with_cursor(self.cursor)
            .render(inner_area, buf);
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
        self.set_col(col);
        self.set_line(line);
    }

    /// Set the column position of the cursor.
    #[inline(always)]
    fn set_col(&mut self, col: usize) {
        self.col = col.min(u16::MAX as usize) as u16;
    }

    /// Set the line position of the cursor.
    #[inline(always)]
    fn set_line(&mut self, line: usize) {
        self.line = line.min(u16::MAX as usize) as u16;
    }

    /// Moves the cursor one cell to the right.
    #[inline(always)]
    fn move_right(&mut self) {
        self.col = self.col.saturating_add(1)
    }

    /// Moves the cursor one cell to the left.
    #[inline(always)]
    fn move_left(&mut self) {
        self.col = self.col.saturating_sub(1)
    }

    /// Moves the cursor one cell up.
    #[inline(always)]
    fn move_up(&mut self) {
        self.line = self.line.saturating_sub(1)
    }

    /// Moves the cursor one cell down.
    #[inline(always)]
    fn move_down(&mut self) {
        self.line = self.line.saturating_add(1)
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

/// A block of text composed with a margin and a the text block itself.
struct EditorBox<'a> {
    text: Text<'a>,
    cursor: Option<Cursor>,
}

impl<'a> EditorBox<'a> {
    /// Creates a new [`EditorBox`] from an iterator of object convertible to a `&str`.
    fn from_iter<I: Iterator<Item = &'a T>, T: AsRef<str> + ?Sized + 'a>(it: I) -> Self {
        let text = Text::from_iter(it.map(|i| i.as_ref()));
        Self { text, cursor: None }
    }

    /// Add a cursor to be rendered.
    fn with_cursor(mut self, cursor: Cursor) -> Self {
        self.cursor = Some(cursor);
        self
    }
}

impl Widget for EditorBox<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let nb_lines = self.text.iter().len();
        let margin_width = nb_lines.to_string().len();
        let lines = if nb_lines == 0 {
            vec![String::from("0")]
        } else {
            (0..nb_lines)
                .map(|n| format!("{n:>margin_width$}"))
                .collect::<Vec<_>>()
        };

        let layout = Layout::horizontal([
            Constraint::Length(margin_width as u16 + 1),
            Constraint::Fill(1),
        ]);
        let [margin_area, text_area] = area.layout(&layout);
        let margin_text = Text::from_iter(lines.iter().map(|s| s.as_str()));

        Block::new()
            .borders(Borders::RIGHT)
            .render(margin_area, buf);
        Paragraph::new(margin_text).render(margin_area, buf);

        Paragraph::new(self.text).render(text_area, buf);

        if let Some(cursor) = self.cursor {
            cursor.render(text_area, buf);
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

        #[test]
        fn move_up() {
            let truth = Cursor {
                line: 0,
                col: 0,
                visible: false,
            };
            let mut cursor = Cursor {
                line: 1,
                col: 0,
                visible: false,
            };

            cursor.move_up();
            assert_eq!(cursor, truth);
        }

        #[test]
        fn move_down() {
            let truth = Cursor {
                line: 1,
                col: 0,
                visible: false,
            };
            let mut cursor = Cursor {
                line: 0,
                col: 0,
                visible: false,
            };

            cursor.move_down();
            assert_eq!(cursor, truth);
        }
    }
}
