use ratatui::{Frame, layout::Rect};

use crate::{
    app::App,
    tui::panes::{Pane, ProcessListOrTree},
};

pub fn render_process_list_tab(app: &mut App, frame: &mut Frame, area: Rect) {
    match &mut app.panes.processes {
        ProcessListOrTree::List(processes) => {
            processes.render(frame, area);
        }
        ProcessListOrTree::Tree(processes) => {
            processes.render(frame, area);
        }
    }
}
