use std::vec;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{info::shared::system_usage_info::SystemUsageInfo, tui::panes::Pane};

// TODO: common trait for widgets
impl Pane for SystemUsageInfo {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let blocks_info = vec![
            Line::from(vec![
                "CPU".cyan(),
                Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                self.total_cpu_usage
                    .to_string()
                    .fg(Color::Rgb(130, 130, 130)),
            ]),
            Line::from(vec![
                "GPU".cyan(),
                Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                self.total_gpu_usage
                    .to_string()
                    .fg(Color::Rgb(130, 130, 130)),
            ]),
            Line::from(vec![
                "Memory".cyan(),
                Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                self.total_memory_usage_mb
                    .to_string()
                    .fg(Color::Rgb(130, 130, 130)),
            ]),
        ];
        // let layout = Layout::horizontal([Constraint::Max(blocks_info.len() as u16 + 2)]);

        frame.render_widget(Paragraph::new(blocks_info), area);
    }
}
