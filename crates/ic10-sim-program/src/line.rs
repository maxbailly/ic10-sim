/// Represents a line in IC10 program.
///
/// IC10 lines are UTF-8 string that can't exceed 90 characters.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Line {
    inner: String,
}

impl Line {
    /// The maximum size of a [`Line`].
    pub const MAX_LENGTH: usize = 90;

    /// Returns a new empty [`Line`].
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a new character `c` at position `pos` in the line.
    ///
    /// If the given position exceeds the length of the line, the character is appended
    /// to the line.
    ///
    /// If the line is full, the character isn't inserted.
    // TODO: if the line is full, we might want to return the non-inserted char in an option.
    #[inline]
    pub fn insert_char_at(&mut self, pos: usize, c: char) {
        if self.is_full() {
            return;
        }

        let pos = self.len().min(pos);
        self.inner.insert(pos, c);
    }

    /// Returns the number of unicode characters in the string.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.chars().count()
    }

    /// Returns `true` if the line is empty, `false` otherwise.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns `true` if the line is full, `false` otherwise.
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.len() >= Self::MAX_LENGTH
    }
}

impl Default for Line {
    fn default() -> Self {
        Self {
            inner: String::with_capacity(Self::MAX_LENGTH),
        }
    }
}

impl TryFrom<&str> for Line {
    // TODO
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.chars().count() > 90 {
            return Err(());
        }

        Ok(Self {
            inner: s.to_string(),
        })
    }
}

impl PartialEq<&str> for Line {
    fn eq(&self, other: &&str) -> bool {
        self.inner == *other
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner)
    }
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner)
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let line = Line::default();

        assert!(line.inner.is_empty());
        assert_eq!(line.inner.capacity(), Line::MAX_LENGTH);
    }

    #[test]
    fn try_from_valid() {
        let truth = "Hello, World! 😀";
        let line = Line::try_from(truth).expect("valid line");

        assert_eq!(line.len(), truth.chars().count());
        assert_eq!(line, "Hello, World! 😀");
    }

    #[test]
    fn try_from_invalid() {
        let truth = std::iter::repeat_n('a', Line::MAX_LENGTH + 1).collect::<String>();
        Line::try_from(truth.as_str()).expect_err("invalid line: string is too long");
    }

    #[test]
    fn insert_char_at() {
        let truth = "Hello, World!";
        let mut line = Line::try_from(truth).expect("valid line");

        line.insert_char_at(5, '😀');
        assert_eq!(line.len(), truth.chars().count() + 1);
        assert_eq!(line, "Hello😀, World!")
    }

    #[test]
    fn insert_char_at_end() {
        let truth = "Hello, World!";
        let mut line = Line::try_from(truth).expect("valid line");

        line.insert_char_at(Line::MAX_LENGTH + 1, '😀');
        assert_eq!(line.len(), truth.chars().count() + 1);
        assert_eq!(line, "Hello, World!😀")
    }

    #[test]
    fn insert_char_at_empty_line() {
        let mut line = Line::new();

        line.insert_char_at(0, '😀');
        assert_eq!(line.len(), 1);
        assert_eq!(line, "😀");
    }

    #[test]
    fn insert_char_at_full() {
        let truth = std::iter::repeat_n('a', Line::MAX_LENGTH).collect::<String>();
        let mut line = Line::try_from(truth.as_str()).expect("valid line");

        line.insert_char_at(0, '😀');
        assert_eq!(line.len(), truth.chars().count());
        assert_eq!(line, truth.as_str())
    }
}
