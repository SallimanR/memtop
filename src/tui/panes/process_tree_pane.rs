use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Cell, Row, Table, TableState},
};

use crate::{
    info::linux::process::process_tree::{ProcessTree, TreeNode},
    tui::{panes::Pane, widgets::selectable_table::SelectableTableWidget},
};

#[derive(Debug, Default)]
pub struct ProcessTreePane {
    pub process_tree: SelectableTableWidget<ProcessTree>,
}

impl ProcessTreePane {
    pub fn new() -> Self {
        let mut table_state = TableState::default();
        table_state.select_first();
        table_state.select_first_column();
        Self {
            process_tree: SelectableTableWidget::new(ProcessTree::new()),
        }
    }
}

fn flatten_tree(nodes: &[TreeNode], root_indices: &[usize]) -> Vec<Row<'static>> {
    let mut rows = Vec::new();
    #[inline]
    fn recurse(nodes: &[TreeNode], idx: usize, depth: usize, rows: &mut Vec<Row>) {
        // const BRANCH_END: char = '└';
        // const BRANCH_SPLIT: char = '├';
        // const BRANCH_HORIZONTAL: char = '─';
        // const SPACED_BRANCH_VERTICAL: &str = "│  ";

        let indent = "│  ".repeat(depth);

        let node = &nodes[idx];
        let display_name = format!("{}├─ {}", indent, node.data.name);
        rows.push(Row::new(vec![
            Cell::from(node.data.pid.to_string()),
            Cell::from(display_name),
        ]));
        for &child in &node.children {
            recurse(nodes, child, depth + 1, rows);
        }
    }
    for &root_idx in root_indices {
        recurse(nodes, root_idx, 0, &mut rows);
    }
    rows
}

impl Pane for ProcessTreePane {
    fn render(&mut self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let root_indices: Vec<usize> = self
            .process_tree
            .items
            .iter()
            .enumerate()
            .filter(|(_, node)| {
                let ppid = node.data.ppid;
                // if no node has pid == ppid, it's a root
                !self.process_tree.items.iter().any(|n| n.data.pid == ppid)
            })
            .map(|(i, _)| i)
            .collect();
        // debug_assert!(!root_indices.is_empty());

        let rows = flatten_tree(&self.process_tree.items, &root_indices);

        let header = Row::new(["PID", "name", "Rss"]).style(Style::new().bold());

        let widths = [
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .column_highlight_style(Color::Gray)
            .cell_highlight_style(Style::new().reversed().yellow())
            .highlight_symbol("> ");

        frame.render_stateful_widget(table, area, &mut self.process_tree.state);
    }
}
