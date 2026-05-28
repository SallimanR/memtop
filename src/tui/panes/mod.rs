pub mod process_list_pane;
pub mod process_tree_pane;
pub mod system_usage_pane;

use std::sync::{Arc, atomic::AtomicBool};

use ratatui::{Frame, layout::Rect};

use crate::tui::panes::{
    process_list_pane::ProcessListPane, process_tree_pane::ProcessTreePane,
    system_usage_pane::SystemUsagePane,
};

#[derive(Debug, Default)]
pub struct Panes {
    pub processes: ProcessListOrTree,
    pub processes_tree_mode: Arc<AtomicBool>,
    pub processes_toggle_threads: Arc<AtomicBool>,

    pub system_usage_pane: SystemUsagePane,
}

impl Panes {
    pub fn new() -> Self {
        Self {
            processes: ProcessListOrTree::default(),
            processes_tree_mode: Arc::new(AtomicBool::new(false)),
            processes_toggle_threads: Arc::new(AtomicBool::new(false)),

            system_usage_pane: SystemUsagePane {
                needs_update: true,
                ..Default::default()
            },
        }
    }
}

#[derive(Debug)]
pub enum ProcessListOrTree {
    List(ProcessListPane),
    Tree(ProcessTreePane),
}

impl Default for ProcessListOrTree {
    fn default() -> Self {
        ProcessListOrTree::List(ProcessListPane::new())
    }
}

pub trait Pane {
    fn render(&mut self, frame: &mut Frame, area: Rect);
}
