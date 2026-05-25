pub mod process_list_tab;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
};

#[derive(Debug, Default)]
pub struct TabsPane {
    pub titles: [&'static str; 2],
    pub selected_tab: usize,
}

impl TabsPane {
    pub fn next(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % self.titles.len()
    }
    pub fn previous(&mut self) {
        if self.selected_tab > 0 {
            self.selected_tab -= 1;
        } else {
            self.selected_tab = self.titles.len() - 1;
        }
    }
}

impl TabsPane {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let tabs = ratatui::widgets::Tabs::new(self.titles.iter().map(|v| Line::from(*v)))
            .select(self.selected_tab)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightGreen),
            );
        frame.render_widget(tabs, area);
    }
}
