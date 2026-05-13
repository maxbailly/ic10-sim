use crate::line::{Line, LineError};

/* ---------- */

/// An IC10 program file.
///
/// IC10 programs can contain at most 128 lines of code and a macimum of 4096 characters.
#[derive(Debug)]
pub struct Program {
    lines: Vec<Line>,
}

impl Program {
    /// The maximum amount of lines of code that an IC10 program can contain.
    ///
    /// This includes empty lines, comment lines, etc.
    pub const MAX_LINES: usize = 128;

    /// The maximum amount of character that an IC10 program can contain.
    pub const MAX_CHARACTERS: usize = 4096;

    /// Creates a new, empty [`Program`].
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a program from some stuff that are convertible to [`Line`].
    ///
    /// # Error
    ///
    /// Returns an error if either:
    /// * the number of given lines is greater than the maximum allowed number of line in an IC10 program file,
    /// * the number of characters contained in the given lines exceeds the maximum allowed number of characters in an IC10 program file,
    /// * one line fails to be converted.
    pub fn from_lines(lines: &[&str]) -> Result<Self, ProgramError> {
        if lines.len() > Self::MAX_LINES {
            return Err(ProgramError::TooManyLines);
        }

        let chars_count = lines.iter().map(|line| line.chars().count()).sum::<usize>();
        if chars_count > Self::MAX_CHARACTERS {
            return Err(ProgramError::TooManyChars);
        }

        let mut prog = Self::default();

        lines.iter().enumerate().try_for_each(|(idx, line)| {
            let line: Line = (*line)
                .try_into()
                .map_err(|e| ProgramError::read_line(idx, e))?;
            prog.lines.push(line);
            Ok(())
        })?;

        Ok(prog)
    }

    /// Inserts a character `c` at `line` and `col`.
    ///
    /// Returns `true` if the character is successfully inserted, `false` otherwise.
    ///
    /// # Notes
    ///
    /// * If the line or the program is full, the character ins't inserted, this function does nothing.
    /// * If the program is empty, a new line containing the given character is created at the beginning of the program.
    /// * If the `line` index exceeds the number of lines in the program, the character is inserted in the last line.
    /// * If the `col` index exceeds the length of the line, the character is inserted at the end of the line.
    pub fn insert_char(&mut self, line: usize, col: usize, c: char) -> bool {
        if self.lines.is_empty() {
            let new_line = self.lines.push_mut(Line::default());
            new_line.insert_char_at(0, c);
            return true;
        }

        if self.char_count() >= Self::MAX_CHARACTERS {
            return false;
        }

        let nb_lines = self.lines.len();
        let line_idx = if line >= nb_lines { nb_lines - 1 } else { line };
        let line = &mut self.lines[line_idx];

        line.insert_char_at(col, c)
    }

    /// Inserts a new line at the given index.
    ///
    /// If the program is full, this function does nothing.
    ///
    /// If the `line` index exceeds the number of lines in the program, the new line is inserted at the end.
    ///
    /// Returns `true` if the line is successfully added, `false` otherwise.
    #[inline(always)]
    pub fn insert_new_line(&mut self, line: usize) -> bool {
        if self.lines.len() >= Self::MAX_LINES {
            return false;
        }

        let idx = line.min(self.lines.len());
        self.lines.insert(idx, Line::default());

        true
    }

    /// Removes a character at the `col` index in the given `line`.
    ///
    /// If the program or the line is empty, this function does nothing.
    ///
    /// If the `line` index exceeds the number of lines in the program, the character will be removed from the last line.
    /// Similarily, if the `col` index exceeds the line's length, the last character will be removed from it.
    ///
    /// Returns `true` if the character is successfully removed, `false` otherwise.
    pub fn remove_char(&mut self, line: usize, col: usize) -> bool {
        if self.lines.is_empty() {
            return false;
        }

        let nb_lines = self.lines.len();
        let line_idx = if line >= nb_lines { nb_lines - 1 } else { line };
        let line = &mut self.lines[line_idx];

        line.remove_char_at(col)
    }

    /// Returns an iterator over the entire lines contained in the program.
    pub fn lines(&self) -> std::slice::Iter<'_, Line> {
        self.lines.iter()
    }

    /// Returns the number of character in the program.
    #[inline(always)]
    fn char_count(&self) -> usize {
        self.lines.iter().map(|line| line.len()).sum::<usize>()
    }
}

impl Default for Program {
    #[inline(always)]
    fn default() -> Self {
        Self {
            lines: Vec::with_capacity(Self::MAX_LINES),
        }
    }
}

/* ---------- */

/// Errors that might happens when dealing with a [`Program`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProgramError {
    /// Error that raises when creating a [`Program`] from some stuff convertible to [`Line`].
    ReadLine { line_nb: usize, error: LineError },
    /// Error that raises when the number of given lines exceeds the maximum allowed number of lines in a IC10 program.
    TooManyLines,
    /// Error that raises when the number of characters in given the given lines exceeds the maximum allowed number of characters in a IC10 program.
    TooManyChars,
}

impl ProgramError {
    /// A line failed to be converted to a [`Line`].
    #[inline(always)]
    fn read_line(line_nb: usize, error: LineError) -> Self {
        Self::ReadLine { line_nb, error }
    }
}

impl std::error::Error for ProgramError {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReadLine { error, .. } => Some(error),
            _ => None,
        }
    }
}

impl std::fmt::Display for ProgramError {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadLine { line_nb, error } => {
                write!(f, "Failed to read line #{}: {}", line_nb, error)
            }
            Self::TooManyLines => write!(f, "Too many lines"),
            Self::TooManyChars => write!(f, "Too many characters"),
        }
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let prog = Program::new();

        assert_eq!(prog.lines.len(), 0);
        assert_eq!(prog.lines.capacity(), Program::MAX_LINES);
    }

    #[test]
    fn from_lines_valid() {
        let truth = ["Hello", "World!"];
        let prog = Program::from_lines(&truth).expect("valid lines");

        assert_eq!(prog.lines.len(), 2);
        assert_eq!(prog.lines, truth);
    }

    #[test]
    fn from_lines_valid_lines_limit() {
        let truth = ["a"; Program::MAX_LINES];
        let prog = Program::from_lines(&truth).expect("valid lines");

        assert_eq!(prog.lines.len(), Program::MAX_LINES);
        assert_eq!(prog.lines, truth);
    }

    #[test]
    fn from_lines_valid_chars_limit() {
        const NLINES: usize = 64;
        let s: String = std::iter::repeat_n('😀', Program::MAX_CHARACTERS / NLINES).collect();
        let truth: Vec<&str> = std::iter::repeat_n(s.as_str(), NLINES).collect();
        let prog = Program::from_lines(&truth).expect("valid lines");

        assert_eq!(prog.lines.len(), NLINES);
        assert_eq!(prog.lines, truth);
    }

    #[test]
    fn from_lines_invalid_too_many_lines() {
        let truth = ["a"; Program::MAX_LINES + 1];
        let err = Program::from_lines(&truth).expect_err("should fail: contains too many lines");

        assert_eq!(err, ProgramError::TooManyLines);
    }

    #[test]
    fn from_lines_invalid_too_many_chars() {
        const NLINES: usize = 64;
        let s: String = std::iter::repeat_n('a', Program::MAX_CHARACTERS / NLINES).collect();
        let mut truth: Vec<&str> = std::iter::repeat_n(s.as_str(), NLINES).collect();
        truth.push("a");

        let err = Program::from_lines(&truth).expect_err("should fail: too many chars");
        assert_eq!(err, ProgramError::TooManyChars);
    }

    #[test]
    fn from_lines_invalid_line_too_long() {
        let truth = [
            "a",
            "loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong",
            "line",
        ];
        let err = Program::from_lines(&truth).expect_err("should fail: a line is too long");

        assert_eq!(
            err,
            ProgramError::ReadLine {
                line_nb: 1,
                error: LineError::FromStrTooLong
            }
        );
    }

    #[test]
    fn insert_char() {
        let mut prog = Program::from_lines(&["Hllo", "World!"]).expect("valid lines");

        let ret = prog.insert_char(0, 1, 'e');
        assert!(ret);
        assert_eq!(prog.lines, ["Hello", "World!"]);
    }

    #[test]
    fn insert_char_empty_program() {
        let mut prog = Program::default();

        let ret = prog.insert_char(1, 1, 'a');
        assert!(ret);
        assert_eq!(prog.lines, ["a"]);
    }

    #[test]
    fn insert_char_index_too_large() {
        let mut prog = Program::from_lines(&["Hello", "World"]).expect("valid lines");

        let ret = prog.insert_char(Program::MAX_LINES + 1, Line::MAX_LENGTH + 1, '!');
        assert!(ret);
        assert_eq!(prog.lines, ["Hello", "World!"]);
    }

    #[test]
    fn insert_char_program_full() {
        const NLINES: usize = 64;
        let s: String = std::iter::repeat_n('😀', Program::MAX_CHARACTERS / NLINES).collect();
        let truth: Vec<&str> = std::iter::repeat_n(s.as_str(), NLINES).collect();
        let mut prog = Program::from_lines(&truth).expect("valid lines");

        let ret = prog.insert_char(0, 0, 'a');
        assert!(!ret);
        assert_eq!(prog.lines, truth);
    }

    #[test]
    fn insert_char_line_full() {
        let s: String = std::iter::repeat_n('a', Line::MAX_LENGTH).collect();
        let truth = [s.as_str()];
        let mut prog = Program::from_lines(&truth).expect("valid line");

        let ret = prog.insert_char(0, 0, 'a');
        assert!(!ret);
        assert_eq!(prog.lines, truth);
    }

    #[test]
    fn insert_line() {
        let mut prog = Program::from_lines(&["Hello", "World"]).expect("valid lines");

        let ret = prog.insert_new_line(1);
        assert!(ret);
        assert_eq!(prog.lines, ["Hello", "", "World"]);
    }

    #[test]
    fn insert_line_empty_program() {
        let mut prog = Program::default();

        let ret = prog.insert_new_line(1);
        assert!(ret);
        assert_eq!(prog.lines, [""]);
    }

    #[test]
    fn insert_line_program_full() {
        let truth = std::iter::repeat_n("", Program::MAX_LINES).collect::<Vec<_>>();
        let mut prog = Program::from_lines(&truth).expect("valid lines");

        let ret = prog.insert_new_line(0);
        assert!(!ret);
        assert_eq!(prog.lines, truth);
    }

    #[test]
    fn insert_line_index_too_large() {
        let mut prog = Program::from_lines(&["Hello", "World"]).expect("valid lines");

        let ret = prog.insert_new_line(Program::MAX_LINES);
        assert!(ret);
        assert_eq!(prog.lines, &["Hello", "World", ""]);
    }

    #[test]
    fn remove_char() {
        let mut prog = Program::from_lines(&["Hello", "World!"]).expect("valid lines");

        prog.remove_char(0, 0);
        assert_eq!(prog.lines, &["ello", "World!"]);
    }

    #[test]
    fn remove_char_indexes_too_big() {
        let mut prog = Program::from_lines(&["Hello", "World!"]).expect("valid lines");

        prog.remove_char(Program::MAX_LINES, Line::MAX_LENGTH);
        assert_eq!(prog.lines, &["Hello", "World"]);
    }

    #[test]
    fn remove_char_empty_program() {
        let mut prog = Program::default();

        prog.remove_char(Program::MAX_LINES, Line::MAX_LENGTH);
        assert!(prog.lines.is_empty());
    }
}
