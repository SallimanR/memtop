use std::{
    io::Stdout,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
        mpsc::{self, Sender},
    },
    thread::{self, JoinHandle},
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
        linux::process::{process_list::ProcessList, process_tree::ProcessTree},
        shared::system_usage::{SystemUsage, SystemUsageFunctionality},
    },
    tui::{
        self,
        panes::{
            ProcessListOrTree, process_list_pane::ProcessListPane,
            process_tree_pane::ProcessTreePane,
        },
    },
};

pub struct UpdateInfo {
    pub system_usage_info: SystemUsage,
    pub process_list: ProcessList,
    pub process_tree: ProcessTree,
}

pub fn start_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    let update_period_ms = Arc::new(AtomicU64::new(500));

    let (sender, receiver) = mpsc::channel::<UpdateInfo>();

    let update_period_ms_thread = update_period_ms.clone();
    let _update_info_thread = crate_update_info_thread(sender, update_period_ms_thread);

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
                KeyCode::Char('c') => {
                    match app.panes.processes {
                        ProcessListOrTree::List(_) => {
                            let mut p = ProcessTreePane::new();
                            p.process_tree.items.update();
                            app.panes.processes = ProcessListOrTree::Tree(p);
                        }
                        ProcessListOrTree::Tree(_) => {
                            let mut p = ProcessListPane::new();
                            p.update();
                            app.panes.processes = ProcessListOrTree::List(p);
                        }
                    }
                    needs_redraw = true;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    match &mut app.panes.processes {
                        ProcessListOrTree::List(processes) => {
                            processes.process_list.select_next(1);
                        }
                        ProcessListOrTree::Tree(processes) => {
                            processes.process_tree.select_next(1);
                        }
                    }
                    needs_redraw = true;
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    match &mut app.panes.processes {
                        ProcessListOrTree::List(processes) => {
                            processes.process_list.select_previous(1);
                        }
                        ProcessListOrTree::Tree(processes) => {
                            processes.process_tree.select_previous(1);
                        }
                    }
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
            match &mut app.panes.processes {
                ProcessListOrTree::List(processes) => {
                    processes.process_list.items = update.process_list;
                }
                ProcessListOrTree::Tree(processes) => {
                    processes.process_tree.items = update.process_tree
                }
            }

            needs_redraw = true
        }

        if needs_redraw {
            terminal.draw(|frame| tui::ui::render(&mut app, frame))?;
            needs_redraw = false
        }
    }

    Ok(())
}

fn crate_update_info_thread(
    sender: Sender<UpdateInfo>,
    update_period_ms: Arc<AtomicU64>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut system = sysinfo::System::new_all();
        loop {
            let mut process_list = ProcessList::new();
            process_list.update();
            let mut process_tree = ProcessTree::new();
            process_tree.update();
            let result = UpdateInfo {
                system_usage_info: SystemUsage::update(&mut system),
                process_list,
                process_tree,
            };

            if sender.send(result).is_err() {
                break;
            }
            let sleep_ms = update_period_ms.load(Ordering::Relaxed);
            thread::sleep(Duration::from_millis(sleep_ms));
        }
    })
}
