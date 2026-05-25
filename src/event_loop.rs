use std::{
    io::Stdout,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

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
    let update_period_ms = Arc::new(AtomicU64::new(500));
    let update_period_ms_thread = update_period_ms.clone();
    thread::spawn(move || {
        loop {
            let mut process_list = ProcessList::new();
            process_list.update();
            let result = Update {
                system_usage_info: SystemUsageInfo::update(&mut system),
                process_list,
            };

            let sleep_ms = update_period_ms_thread.load(Ordering::Relaxed);
            thread::sleep(Duration::from_millis(sleep_ms));
            if sender.send(result).is_err() {
                break;
            }
        }
    });

    let mut should_quit = false;
    let mut needs_redraw = true;
    let input_poll_timeout = Duration::from_millis(16);
    while !should_quit {
        if event::poll(input_poll_timeout)?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('+') => {
                    let current = update_period_ms.load(Ordering::Relaxed);
                    let new = (current + 100).min(5000);
                    update_period_ms.store(new, Ordering::Relaxed);
                }
                KeyCode::Char('-') => {
                    let current = update_period_ms.load(Ordering::Relaxed);
                    let new = current.saturating_sub(100).max(100);
                    update_period_ms.store(new, Ordering::Relaxed);
                }
                KeyCode::Char('1') => {
                    app.panes.system_usage_pane.needs_update =
                        !app.panes.system_usage_pane.needs_update;
                    needs_redraw = true;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    app.panes.process_list_pane.process_list.select_next(1);
                    needs_redraw = true;
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    app.panes.process_list_pane.process_list.select_previous(1);
                    needs_redraw = true;
                }
                KeyCode::Char('d') => {
                    app.tabs.next_tab();
                    needs_redraw = true;
                }
                KeyCode::Char('a') => {
                    app.tabs.previous_tab();
                    needs_redraw = true;
                }
                KeyCode::Char('q') | KeyCode::Esc => should_quit = true,
                _ => {}
            }
        }

        if let Ok(update) = receiver.try_recv() {
            app.panes.system_usage_pane.system_usage_info = update.system_usage_info;
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
