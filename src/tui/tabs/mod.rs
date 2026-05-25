pub mod process_list_tab;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
};

pub const TAB_TITLES: [&'static str; 2] = ["Process List", "Process"];
const TAB_COUNT: usize = TAB_TITLES.len();

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Tabs {
    #[default]
    ProcessList,
    Process,
}

impl From<usize> for Tabs {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::ProcessList,
            1 => Self::Process,
            _ => Self::default(),
        }
    }
}

impl Tabs {
    pub fn next(self) -> Self {
        let current = self as usize;
        let next = (current + 1) % TAB_COUNT;
        Tabs::from(next)
    }

    pub fn previous(self) -> Self {
        let current = self as usize;
        let prev = (current + TAB_COUNT - 1) % TAB_COUNT;
        Tabs::from(prev)
    }
}

#[derive(Debug, Default)]
pub struct TabsPane {
    pub titles: [&'static str; 2],
    pub selected_tab: Tabs,
}

impl TabsPane {
    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }
    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }
}

impl TabsPane {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let tabs = ratatui::widgets::Tabs::new(self.titles.iter().map(|v| Line::from(*v)))
            .select(self.selected_tab as usize)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightGreen),
            );
        frame.render_widget(tabs, area);
    }
}
