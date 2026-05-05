use crate::{Line, line::LineError};

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
        let s: String = std::iter::repeat_n('a', Program::MAX_CHARACTERS / NLINES).collect();
        let truth: Vec<&str> = std::iter::repeat_n(s.as_str(), NLINES).collect();
        let prog = Program::from_lines(&truth).expect("valid lines");

        assert_eq!(prog.lines.len(), NLINES);
        assert_eq!(prog.lines, truth);
    }

    #[test]
    fn from_lines_invalid_too_many_lines() {
        let truth = ["a"; Program::MAX_LINES + 1];
        let err = Program::from_lines(&truth).expect_err("should fail: contains too many lines");

        assert_eq!(err, ProgramError::TooManyLines)
    }

    #[test]
    fn from_lines_invalid_too_many_chars() {
        const NLINES: usize = 64;
        let s: String = std::iter::repeat_n('a', Program::MAX_CHARACTERS / NLINES).collect();
        let mut truth: Vec<&str> = std::iter::repeat_n(s.as_str(), NLINES).collect();
        truth.push("a");

        let err = Program::from_lines(&truth).expect_err("should fail: too many chars");
        assert_eq!(err, ProgramError::TooManyChars)
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
        )
    }
}
