use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::{Block, BorderType, Widget};

/* ---------- */

#[derive(Debug, Default)]
pub(crate) struct Editor {}

impl Editor {
    /// Returns a new default [`Editor`] widget.
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self::default()
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

        border.render(area, buf);
    }
}
