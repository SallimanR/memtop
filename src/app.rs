use crate::{
    event_loop::start_event_loop,
    info::shared::system_usage_info::SystemUsageInfo,
    tui::{panes::Panes, tabs::TabsPane},
};

#[derive(Debug, Default)]
pub struct App {
    pub tabs: TabsPane,
    pub panes: Panes,
}

impl App {
    pub fn new() -> Self {
        Self {
            panes: Panes::new(),
            tabs: TabsPane {
                titles: ["Process List", "Process"],
                selected_tab: 0,
            },
            ..Default::default()
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(start_event_loop)
}
