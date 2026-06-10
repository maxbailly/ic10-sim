mod actions;
mod app;
mod chip;
mod editor;
mod too_small;

/* ---------- */

fn main() {
    // TODO: error management
    let _ = ratatui::run(app::run);
}
