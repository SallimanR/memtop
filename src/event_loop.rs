use std::{io::Stdout, sync::mpsc, thread, time::Duration};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::event::{self, Event, KeyCode},
};

use crate::{
    app::App,
    info::{
        linux::process::process_list::ProcessList,
        shared::system_usage_info::{SystemUsageInfo, SystemUsageInfoFunctionality},
    },
    tui,
};

pub struct Update {
    pub system_usage_info: SystemUsageInfo,
    pub process_list: ProcessList,
}

pub fn start_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    let mut system = sysinfo::System::new_all();

    let (sender, receiver) = mpsc::channel::<Update>();
    let update_period = Duration::from_millis(500);
    thread::spawn(move || {
        loop {
            let mut process_list = ProcessList::new();
            process_list.update();
            let result = Update {
                system_usage_info: SystemUsageInfo::update(&mut system),
                process_list,
            };
            thread::sleep(update_period);
            if sender.send(result).is_err() {
                break;
            }
        }
    });

    let mut should_quit = false;
    let mut needs_redraw = true;
    let poll_timeout = Duration::from_millis(16);
    while !should_quit {
        if event::poll(poll_timeout)?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    app.panes.process_list_pane.process_list.select_next(1);
                    needs_redraw = true;
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    app.panes.process_list_pane.process_list.select_previous(1);
                    needs_redraw = true;
                }
                KeyCode::Char('d') => {
                    app.tabs.next();
                    needs_redraw = true;
                }
                KeyCode::Char('a') => {
                    app.tabs.previous();
                    needs_redraw = true;
                }
                KeyCode::Char('q') | KeyCode::Esc => should_quit = true,
                _ => {}
            }
        }

        if let Ok(update) = receiver.try_recv() {
            app.system_usage_info = update.system_usage_info;
            app.panes.process_list_pane.process_list.items = update.process_list;
            needs_redraw = true
        }

        if needs_redraw {
            terminal.draw(|frame| tui::ui::render(&mut app, frame))?;
            needs_redraw = false
        }
    }

    Ok(())
}
