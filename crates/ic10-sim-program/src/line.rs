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

        if pos >= self.len() {
            self.inner.push(c);
            return;
        }

        let pos = self.get_char_true_pos_at(pos).expect("Out of bound access");
        self.inner.insert(pos, c);
    }

    /// Removes a character at `pos` from the string.
    ///
    /// If the `pos` index is greater than the line's length, the last character is removed
    /// from the line.
    ///
    /// This function does nothing if the line is empty.
    pub fn remove_char_at(&mut self, pos: usize) {
        if self.is_empty() {
            return;
        }

        let len = self.len();
        let pos = if pos >= len { len - 1 } else { pos };
        let pos = self
            .get_char_true_pos_at(pos)
            .expect("Out of line's bound access");

        self.inner.remove(pos);
    }

    /// Replaces the character at the position `pos` by the given character `c`.
    ///
    /// If the position is greater than the line's length, this function does nothing.
    pub fn replace_char_at(&mut self, pos: usize, c: char) {
        let Some((pos, to_replace)) = self.inner.char_indices().nth(pos) else {
            return;
        };

        self.inner
            .replace_range(pos..pos + to_replace.len_utf8(), &c.to_string());
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

    /// Returns the true position of the character at the given `pos` with respect to the UTF-8 character boundary.
    ///
    /// Returns `None` if the given position is out of the line's bounds.
    #[inline]
    fn get_char_true_pos_at(&self, pos: usize) -> Option<usize> {
        self.inner.char_indices().nth(pos).map(|(pos, _)| pos)
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

    #[test]
    fn remove_char_at() {
        let base = "Hello😀, World!";
        let mut line = Line::try_from(base).expect("valid line");

        line.remove_char_at(6);
        assert_eq!(line.len(), base.chars().count() - 1);
        assert_eq!(line, "Hello😀 World!");
    }

    #[test]
    fn remove_char_at_empty() {
        let mut line = Line::default();

        line.remove_char_at(0);
        assert!(line.is_empty())
    }

    #[test]
    fn remove_char_at_big_idx() {
        let base = "Hello😀, World!";
        let mut line = Line::try_from(base).expect("valid line");

        line.remove_char_at(Line::MAX_LENGTH);
        assert_eq!(line.len(), base.chars().count() - 1);
        assert_eq!(line, "Hello😀, World");
    }

    #[test]
    fn replace_char_at() {
        let base = "Hell😀, World!";
        let mut line = Line::try_from(base).expect("valid line");

        line.replace_char_at(4, 'o');
        assert_eq!(line.len(), base.chars().count());
        assert_eq!(line, "Hello, World!");
    }

    #[test]
    fn replace_char_at_out_of_bounds() {
        let base = "Hello, World!";
        let mut line = Line::try_from(base).expect("valid line");

        line.replace_char_at(Line::MAX_LENGTH, 'o');
        assert_eq!(line.len(), base.chars().count());
        assert_eq!(line, "Hello, World!");
    }

    #[test]
    fn replace_char_at_empty() {
        let mut line = Line::default();

        line.replace_char_at(0, 'o');
        assert!(line.is_empty());
    }

    #[test]
    fn get_char_true_pos_at() {
        let base = "😀a";
        let line = Line::try_from(base).expect("valid line");

        assert_eq!(line.get_char_true_pos_at(0), Some(0));
        assert_eq!(line.get_char_true_pos_at(1), Some(4));
        assert_eq!(line.get_char_true_pos_at(2), None);
    }
}
