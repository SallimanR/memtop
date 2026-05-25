use crate::{
    event_loop::start_event_loop,
    tui::{
        panes::Panes,
        tabs::{Tabs, TabsPane},
    },
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
                selected_tab: Tabs::ProcessList,
            },
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(start_event_loop)
}
