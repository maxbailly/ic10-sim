mod actions;
mod app;
mod editor;

/* ---------- */

fn main() {
    // TODO: error management
    let _ = ratatui::run(app::run);
}
