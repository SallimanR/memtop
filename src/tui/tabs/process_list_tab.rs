use ratatui::{Frame, layout::Rect};

use crate::{
    app::App,
    tui::panes::{Pane, ProcessListOrTree},
};

pub fn render_process_list_tab(app: &mut App, frame: &mut Frame, area: Rect) {
    // app.panes.process_list_pane.render(frame, area);
    // app.panes.process_tree_pane.render(frame, area);
    match &mut app.panes.processes {
        ProcessListOrTree::List(processes) => {
            processes.render(frame, area);
        }
        ProcessListOrTree::Tree(processes) => {
            processes.render(frame, area);
        }
    }
}
