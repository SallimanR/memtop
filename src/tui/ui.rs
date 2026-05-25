use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    app::App,
    tui::{panes::Pane, tabs},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    #[cfg(feature = "profile-with-tracy")]
    let _span = tracy_client::span!("ui::render");

    let chunks = get_tui_layout(app, frame);

    app.panes.system_usage_pane.render(frame, chunks[0]);

    app.tabs.render(frame, chunks[1]);
    match app.tabs.selected_tab {
        0 => {
            tabs::process_list_tab::render_process_list_tab(app, frame, chunks[2]);
        }
        1 => {}
        _ => {}
    }
}

fn get_tui_layout(app: &mut App, frame: &mut Frame) -> Rc<[Rect]> {
    let system_usage_pane_offset = if app.panes.system_usage_pane.needs_update {
        3
    } else {
        0
    };

    Layout::vertical([
        Constraint::Length(system_usage_pane_offset),
        Constraint::Length(2),
        Constraint::Percentage(90),
    ])
    .split(frame.area())
}
