use std::vec;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{info::shared::system_usage::SystemUsage, tui::panes::Pane};

#[derive(Debug, Default)]
pub struct SystemUsagePane {
    pub system_usage_info: SystemUsage,
    pub memory_displayed_unit: MemoryUnit,
    pub needs_update: bool,
}

#[derive(Debug, Default)]
pub enum MemoryUnit {
    B,
    #[default]
    KB,
    MB,
    GB,
}

impl MemoryUnit {
    fn to_bytes(&self) -> u64 {
        match self {
            MemoryUnit::B => 1,
            MemoryUnit::KB => 1024,
            MemoryUnit::MB => 1024 * 1024,
            MemoryUnit::GB => 1024 * 1024 * 1024,
        }
    }

    fn get_label(&self) -> &'static str {
        match self {
            MemoryUnit::B => "B",
            MemoryUnit::KB => "KB",
            MemoryUnit::MB => "MB",
            MemoryUnit::GB => "GB",
        }
    }
}

// TODO: common trait for widgets
impl Pane for SystemUsagePane {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let mem_usage =
            self.system_usage_info.total_memory_usage_bytes / self.memory_displayed_unit.to_bytes();
        let blocks_info = vec![
            Line::from(vec![
                "CPU".cyan(),
                Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                self.system_usage_info
                    .total_cpu_usage
                    .to_string()
                    .fg(Color::Rgb(130, 130, 130)),
                "%".cyan(),
            ]),
            Line::from(vec![
                "GPU".cyan(),
                Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                self.system_usage_info
                    .total_gpu_usage
                    .to_string()
                    .fg(Color::Rgb(130, 130, 130)),
                "%".cyan(),
            ]),
            Line::from(vec![
                "Memory".cyan(),
                Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                mem_usage.to_string().fg(Color::Rgb(130, 130, 130)),
                Span::raw(self.memory_displayed_unit.get_label()).cyan(),
            ]),
        ];

        frame.render_widget(Paragraph::new(blocks_info), area);
    }
}
