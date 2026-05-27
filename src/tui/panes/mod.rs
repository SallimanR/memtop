pub mod process_list_pane;
pub mod process_tree_pane;
pub mod system_usage_pane;

use ratatui::{Frame, layout::Rect};

use crate::{
    info::{linux::process::process_tree::ProcessTree, shared::system_usage::SystemUsage},
    tui::{
        panes::{
            process_list_pane::ProcessListPane, process_tree_pane::ProcessTreePane,
            system_usage_pane::SystemUsagePane,
        },
        widgets::selectable_table::SelectableTableWidget,
    },
};

#[derive(Debug, Default)]
pub struct Panes {
    pub processes: ProcessListOrTree,
    pub system_usage_pane: SystemUsagePane,
}

impl Panes {
    pub fn new() -> Self {
        Self {
            processes: ProcessListOrTree::default(),
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
