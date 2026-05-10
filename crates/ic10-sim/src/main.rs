mod actions;
mod app;

/* ---------- */

fn main() {
    // TODO: error management
    let _ = ratatui::run(app::run);
}
