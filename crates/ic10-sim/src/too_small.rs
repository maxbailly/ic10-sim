use ratatui::layout::Constraint;
use ratatui::prelude::{Buffer, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};

/* ---------- */

/// A popup that is shown to the user when the terminal is too small
/// for the simulator io be somewhat usable.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct TooSmall;

impl Widget for TooSmall {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let area = area.centered(Constraint::Length(60), Constraint::Length(10));

        // TODO: Print an error message in the popup telling that something went wrong
        // when trying to get the terminal current size.
        let (term_width, term_height) = ratatui::crossterm::terminal::size().unwrap_or_default();
        let curr_term_size_string =
            format!("Current terminal size: W: {term_width} * H: {term_height}");

        let lines = [
            "Your terminal is a bit too small",
            "Please resize your terminal to at least W: 130 * H: 40",
            "",
            &curr_term_size_string,
        ];

        let max_line_len = lines
            .iter()
            .map(|s| s.chars().count())
            .max()
            .expect("Should not be empty");

        let block = Block::bordered().border_type(BorderType::Rounded);
        let inner_area = block.inner(area);
        let inner_area = inner_area.centered(
            Constraint::Length(max_line_len as u16),
            Constraint::Length(4),
        );
        block.render(area, buf);

        let text = Text::from_iter(lines).centered();
        Paragraph::new(text).centered().render(inner_area, buf);
    }
}
