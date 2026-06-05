use ratatui::layout::Constraint::Fill;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::{Block, Widget};

/* ---------- */

/// Widget to display the chip's registers and stack values.
#[derive(Debug, Default)]
pub(crate) struct Chip {}

impl Widget for &Chip {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [registers_area, stack_area] =
            Layout::vertical([Constraint::Length(20), Fill(3)]).areas(area);

        let registers_block = Block::bordered().title(" Registers ");
        let _regsiters_inner_area = registers_block.inner(area);
        registers_block.render(registers_area, buf);

        let stack_block = Block::bordered().title(" Stack ");
        let _stack_inner_area = stack_block.inner(area);
        stack_block.render(stack_area, buf);
    }
}
