use ratatui::{Frame, layout::Rect};

use crate::{app::App, tui::panes::Pane};

pub fn render_process_list_tab(app: &mut App, frame: &mut Frame, area: Rect) {
    app.panes.process_list_pane.render(frame, area);
}
