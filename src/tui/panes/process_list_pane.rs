use std::ops::Deref;

use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Row, Table, TableState},
};

use crate::{
    info::linux::process::process_list::ProcessList,
    tui::{panes::Pane, widgets::selectable_table::SelectableTableWidget},
};

#[derive(Debug, Default)]
pub struct ProcessListPane {
    pub process_list: SelectableTableWidget<ProcessList>,
}

impl ProcessListPane {
    pub fn new() -> Self {
        let mut table_state = TableState::default();
        table_state.select_first();
        table_state.select_first_column();
        Self {
            process_list: SelectableTableWidget::new(ProcessList::new()),
        }
    }
    pub fn update(&mut self) {
        self.process_list.items.update();
    }
}

impl Pane for ProcessListPane {
    fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let header = Row::new(["PID", "name", "Rss", "Pss"]).style(Style::new().bold());

        let mut rows: Vec<Row> = Vec::with_capacity(self.process_list.items.len());
        for process in self.process_list.items.deref() {
            rows.push(Row::new([
                process.pid.to_string(),
                process.name.to_string(),
                process.pss.to_string(),
                process.rss.to_string(),
            ]))
        }
        let widths = [
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
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

        frame.render_stateful_widget(table, area, &mut self.process_list.state);
    }
}
