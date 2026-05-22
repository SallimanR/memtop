use std::{fs, ops::Deref};

use crate::info::linux::process::Process;

#[derive(Debug, Default)]
pub struct ProcessList(pub Vec<Process>);

impl Deref for ProcessList {
    type Target = Vec<Process>;
    fn deref(&self) -> &Vec<Process> {
        &self.0
    }
}

impl ProcessList {
    pub fn new() -> Self {
        let mut s = Self(Vec::new());
        s.update();
        s
    }

    pub fn update(&mut self) {
        self.update_with_files();
    }

    fn update_with_files(&mut self) {
        #[cfg(feature = "profile-with-tracy")]
        let _span = tracy_client::span!("ProcessList::update_with_files");

        self.0.clear();

        let files = match fs::read_dir("/proc") {
            Ok(files) => files,
            Err(_) => return,
        };

        let mut buf = [0u8; 102400];

        for pid in files.flatten().filter_map(|entry| {
            let name = entry.file_name();
            let s = name.to_str()?;
            s.parse::<u32>().ok()
        }) {
            let name = Process::get_name_by_pid(pid);
            let (rss, pss) = Process::get_smaps_rollup_by_pid_buf(pid, &mut buf);

            let process = Process {
                pid,
                name,
                rss: rss.unwrap_or_default(),
                pss: pss.unwrap_or_default(),
            };
            // let process = Process {
            //     pid,
            //     name,
            //     ..Default::default()
            // };

            self.0.push(process);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut process_list = ProcessList::new();
        process_list.update();
        assert!(!process_list.0.is_empty());

        for process in &process_list.0 {
            println!(
                "pid: {}, name: {}, rss: {}, pss: {}",
                process.pid, process.name, process.rss, process.pss
            )
        }
    }
}
