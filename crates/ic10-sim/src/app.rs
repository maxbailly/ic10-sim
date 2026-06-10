use ratatui::DefaultTerminal;
use ratatui::crossterm;
use ratatui::crossterm::event::Event;
use ratatui::layout::Constraint;
use ratatui::layout::Constraint::Fill;
use ratatui::layout::Layout;
use ratatui::prelude::{Buffer, Rect, Size};
use ratatui::widgets::Widget;

use std::io::Result as IoResult;
use std::time::Duration;

use crate::actions::GlobalAction;
use crate::actions::IdleAction;
use crate::chip::Chip;
use crate::editor::Editor;

/* ---------- */

/// Minimal size for the UI to be kind of usable.
const MIN_TERM_SIZE: Size = Size::new(120, 50);

/* ---------- */

/// Main UI of the app.
pub(crate) struct App {
    /// Is the app currently running?
    running: bool,
    /// Is the terminal size is smaller than [`MIN_TERM_SIZE`]?
    terminal_too_small: bool,
    /// App state.
    state: AppState,

    /// Editor widget.
    editor: Editor,
    /// Chip widget.
    chip: Chip,
}

impl App {
    /// Creates a new default [`App`].
    #[inline(always)]
    fn new() -> Self {
        Self {
            running: true,
            terminal_too_small: false,
            state: AppState::default(),
            editor: Editor::default(),
            chip: Chip::default(),
        }
    }

    /// Updates the UI according the given input.
    #[inline(always)]
    fn update(&mut self, event: Event) {
        // After handling global events, here's no need to handle other events.
        if let Some(global_action) = GlobalAction::from_event(&event) {
            match global_action {
                GlobalAction::Quit => {
                    self.running = false;
                    return;
                }
                GlobalAction::Resize(size) => {
                    self.terminal_too_small =
                        size.height < MIN_TERM_SIZE.height || size.width < MIN_TERM_SIZE.width
                }
            }

            return;
        }

        match self.state {
            AppState::Idle => self.handle_event(event),
            AppState::Editing => self.editor.handle_event(&mut self.state, event),
        }
    }

    /// Handles event happening when the app is idling.
    fn handle_event(&mut self, event: Event) {
        let action = IdleAction::from_event(event);

        match action {
            Some(IdleAction::Quit) => self.running = false,
            Some(IdleAction::StartEdition) => {
                self.state = AppState::Editing;
                self.editor.activate();
            }
            None => (),
        }
    }

    /// Returns `true` if the TUI app is running, `false` otherwise.
    #[inline(always)]
    fn is_running(&self) -> bool {
        self.running
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if self.terminal_too_small {
            // TODO: design the popup that tells the user to resize its terminal
            return;
        }

        let [editor_area, chip_area] =
            Layout::horizontal([Constraint::Min(100), Fill(16)]).areas(area);

        self.editor.render(editor_area, buf);
        self.chip.render(chip_area, buf);
    }
}

/* ---------- */

/// App's main loop.
pub(crate) fn run(terminal: &mut DefaultTerminal) -> IoResult<()> {
    let mut app = App::new();

    while app.is_running() {
        terminal.draw(|frame| frame.render_widget(&app, frame.area()))?;
        let Some(event) = get_event(Duration::from_micros(16700))? else {
            continue;
        };

        if let Some(GlobalAction::Quit) = GlobalAction::from_event(&event) {
            break;
        }

        app.update(event);
    }

    Ok(())
}

/* ---------- */

/// App state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AppState {
    /// The app is idling.
    #[default]
    Idle,
    /// The editor is active.
    Editing,
}

impl AppState {
    pub(crate) fn quit(&mut self) {
        if let Self::Editing = *self {
            *self = Self::Idle
        }
    }
}

/* ---------- */

/// Returns an IO event.
///
/// If the polling times out, the function returns `None`.
///
/// # Errors
///
/// Returns an error if something when wrong when polling or reading an event.
#[inline(always)]
fn get_event(timeout: Duration) -> IoResult<Option<Event>> {
    if !crossterm::event::poll(timeout)? {
        return Ok(None);
    }

    Ok(Some(crossterm::event::read()?))
}
