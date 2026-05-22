use crate::{
    event_loop::start_event_loop, info::shared::system_usage_info::SystemUsageInfo,
    tui::panes::Panes,
};

#[derive(Debug, Default)]
pub struct App {
    pub tabs: Tabs,
    pub panes: Panes,

    pub system_usage_info: SystemUsageInfo,
}

#[derive(Debug, Default)]
pub struct Tabs {
    pub titles: [&'static str; 2],
    pub selected_tab: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            panes: Panes::new(),
            tabs: Tabs {
                titles: ["General", "Processes"],
                selected_tab: 0,
            },
            ..Default::default()
        }
    }
}

impl Tabs {
    pub fn next(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % self.titles.len()
    }
    pub fn previous(&mut self) {
        if self.selected_tab > 0 {
            self.selected_tab -= 1;
        } else {
            self.selected_tab = self.titles.len();
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(start_event_loop)
}
