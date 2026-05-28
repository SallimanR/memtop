use std::ops::Deref;

use crate::info::linux::process;

#[derive(Debug, Default)]
pub struct ProcessInfoLine {
    pub pid: u32,

    pub name: String,

    pub pss: u64,
    pub rss: u64,
}

#[derive(Debug, Default)]
pub struct ProcessList(pub Vec<ProcessInfoLine>);

impl Deref for ProcessList {
    type Target = Vec<ProcessInfoLine>;
    fn deref(&self) -> &Vec<ProcessInfoLine> {
        &self.0
    }
}

impl ProcessList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn update(&mut self, toggle_threads: bool) -> Option<()> {
        self.build_process_list(toggle_threads)
    }

    fn build_process_list(&mut self, toggle_threads: bool) -> Option<()> {
        #[cfg(feature = "profile-with-tracy")]
        let _span = tracy_client::span!("ProcessList::build_process_list");

        self.0.clear();

        let mut buf = [0u8; 102400];

        let pids = process::get_pids_of_all_processes()?;

        for pid in pids {
            let threads_ids = process::proc_get_threads_ids(pid)?;
            for thread_id in threads_ids {
                let name = process::proc_get_name(thread_id);
                // let (rss, pss) = process::proc_get_smaps_rollup_by_pid(thread_id, &mut buf);

                let process = ProcessInfoLine {
                    pid: thread_id,
                    name,
                    ..Default::default() // rss: rss.unwrap_or_default(),
                                         // pss: pss.unwrap_or_default(),
                };
                self.0.push(process);

                if !toggle_threads {
                    break;
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut process_list = ProcessList::new();
        assert_eq!(process_list.update(true), None);
        assert!(!process_list.0.is_empty());

        for process in &process_list.0 {
            println!(
                "pid: {}, name: {}, rss: {}, pss: {}",
                process.pid, process.name, process.rss, process.pss
            )
        }
    }
}
