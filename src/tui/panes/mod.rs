pub mod process_list_pane;
pub mod system_usage_info_pane;

use ratatui::{Frame, layout::Rect};

use crate::tui::panes::process_list_pane::ProcessListPane;

#[derive(Debug, Default)]
pub struct Panes {
    pub process_list_pane: ProcessListPane,
}

impl Panes {
    pub fn new() -> Self {
        Self {
            process_list_pane: ProcessListPane::new(),
        }
    }
}

pub trait Pane {
    fn render(&mut self, frame: &mut Frame, area: Rect);
}
