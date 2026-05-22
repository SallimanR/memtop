use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::Tabs,
};

use crate::{app::App, tui::panes::Pane};

pub fn render(app: &mut App, frame: &mut Frame) {
    #[cfg(feature = "profile-with-tracy")]
    let _span = tracy_client::span!("ui::render");

    let chunks = Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).split(frame.area());
    render_tabs_line(app, frame, chunks[0]);
    match app.tabs.selected_tab {
        0 => {
            render_general_tab(app, frame, chunks[1]);
        }
        1 => {}
        _ => {}
    }
}

fn render_tabs_line(app: &mut App, frame: &mut Frame, area: Rect) {
    let tabs = Tabs::new(app.tabs.titles.iter().map(|v| Line::from(*v)))
        .select(app.tabs.selected_tab)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightGreen),
        );
    frame.render_widget(tabs, area);
}

fn render_general_tab(app: &mut App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).split(area);

    app.system_usage_info.render(frame, chunks[0]);
    app.panes.process_list_pane.render(frame, chunks[1]);
}
