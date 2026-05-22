pub mod process_list;
pub mod smaps_rollup;

use std::fs;

#[derive(Debug, Default)]
pub struct Process {
    pub pid: u32,

    pub name: String,
    pub rss: u64,

    pub pss: u64,
}

impl Process {
    pub fn get_name_by_pid(pid: u32) -> String {
        #[cfg(feature = "profile-with-tracy")]
        let _span = tracy_client::span!("Process::get_name_by_pid");

        let content = fs::read_to_string(format!("/proc/{}/comm", pid));
        match content {
            Ok(info) => info,
            Err(_) => "".to_owned(),
        }
    }
}
