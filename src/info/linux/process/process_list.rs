use std::{fs, ops::Deref};

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
        let mut s = Self(Vec::new());
        s.update();
        s
    }

    pub fn update(&mut self) {
        self.build_process_list();
    }

    fn build_process_list(&mut self) {
        #[cfg(feature = "profile-with-tracy")]
        let _span = tracy_client::span!("ProcessList::build_process_list");

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
            let name = process::proc_get_name_by_pid(pid);
            let (rss, pss) = process::smaps_rollup::proc_get_smaps_rollup_by_pid(pid, &mut buf);

            let process = ProcessInfoLine {
                pid,
                name,
                rss: rss.unwrap_or_default(),
                pss: pss.unwrap_or_default(),
            };

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
