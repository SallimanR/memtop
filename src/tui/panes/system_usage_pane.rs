use std::vec;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{info::shared::system_usage_info::SystemUsageInfo, tui::panes::Pane};

#[derive(Debug, Default)]
pub struct SystemUsagePane {
    pub system_usage_info: SystemUsageInfo,
    pub needs_update: bool,
}

// TODO: common trait for widgets
impl Pane for SystemUsagePane {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let blocks_info = vec![
            Line::from(vec![
                "CPU".cyan(),
                Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                self.system_usage_info
                    .total_cpu_usage
                    .to_string()
                    .fg(Color::Rgb(130, 130, 130)),
            ]),
            Line::from(vec![
                "GPU".cyan(),
                Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                self.system_usage_info
                    .total_gpu_usage
                    .to_string()
                    .fg(Color::Rgb(130, 130, 130)),
            ]),
            Line::from(vec![
                "Memory".cyan(),
                Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                self.system_usage_info
                    .total_memory_usage_mb
                    .to_string()
                    .fg(Color::Rgb(130, 130, 130)),
            ]),
        ];

        frame.render_widget(Paragraph::new(blocks_info), area);
    }
}
