pub mod process_list_pane;
pub mod system_usage_pane;

use ratatui::{Frame, layout::Rect};

use crate::{
    info::shared::system_usage::SystemUsage,
    tui::panes::{process_list_pane::ProcessListPane, system_usage_pane::SystemUsagePane},
};

#[derive(Debug, Default)]
pub struct Panes {
    pub process_list_pane: ProcessListPane,
    pub system_usage_pane: SystemUsagePane,
}

impl Panes {
    pub fn new() -> Self {
        Self {
            process_list_pane: ProcessListPane::new(),
            system_usage_pane: SystemUsagePane {
                needs_update: true,
                ..Default::default()
            },
        }
    }
}

pub trait Pane {
    fn render(&mut self, frame: &mut Frame, area: Rect);
}
