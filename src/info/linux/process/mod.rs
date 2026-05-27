pub mod process_list;
pub mod process_tree;

use std::{
    fs::{self, DirEntry, File, ReadDir},
    io::Read,
    iter::{FilterMap, Flatten},
};

#[derive(Debug, Default)]
pub struct ProcessFullInfo {
    /* IDs */
    pub pid: u32,
    pub ppid: u32,
    pub tgid: u32,

    pub name: String,

    /* Memory */
    pub pss: u64,
    pub rss: u64,
}

pub fn get_pids_of_all_processes()
-> Option<FilterMap<Flatten<ReadDir>, impl FnMut(DirEntry) -> Option<u32>>> {
    let files = fs::read_dir("/proc").ok()?;
    Some(files.flatten().filter_map(|entry| {
        let name = entry.file_name();
        let s = name.to_str()?;
        s.parse::<u32>().ok()
    }))
}

pub fn proc_get_ppid(pid: u32, buf: &mut [u8; 512]) -> Option<u32> {
    let mut f = File::open(format!("/proc/{}/stat", pid)).ok()?;

    let n = f.read(&mut buf[..]).ok()?;
    let content = str::from_utf8(&buf[..n]).ok()?;

    // content = "/proc/{pid}/stat": pid (comm) state ppid ...
    let after_comm = content.rsplit(')').next()?; // read content after ")"
    let mut fields = after_comm.split_whitespace();
    fields.next(); // skip state
    let ppid_str = fields.next()?;
    ppid_str.parse().ok()
}

pub fn proc_get_tgid(pid: u32, buf: &mut [u8; 512]) -> Option<u32> {
    let mut f = File::open(format!("/proc/{}/status", pid)).ok()?;

    let n = f.read(&mut buf[..]).ok()?;
    let content = str::from_utf8(&buf[..n]).ok()?;

    for line in content.lines() {
        if line.starts_with("Tgid:") {
            return line.split_whitespace().nth(1)?.parse().ok();
        }
    }
    None
}

pub fn proc_get_name_by_pid(pid: u32) -> String {
    #[cfg(feature = "profile-with-tracy")]
    let _span = tracy_client::span!("Process::get_name_by_pid");

    let content = fs::read_to_string(format!("/proc/{}/comm", pid));
    match content {
        Ok(info) => info,
        Err(_) => "".to_owned(),
    }
}

pub fn proc_get_smaps_rollup_by_pid(pid: u32, buf: &mut [u8]) -> (Option<u64>, Option<u64>) {
    #[cfg(feature = "profile-with-tracy")]
    let _span = tracy_client::span!("Process::get_smaps_rollup_by_pid_buf");

    let path = format!("/proc/{}/smaps_rollup", pid);
    let mut file = match File::open(&path) {
        Ok(info) => info,
        Err(_) => return (None, None),
    };
    let n = match file.read(buf) {
        Ok(info) => info,
        Err(_) => return (None, None),
    };
    let content = match std::str::from_utf8(&buf[..n]) {
        Ok(info) => info,
        Err(_) => return (None, None),
    };

    let (pss, rss) = parse_rss_and_pss(content.lines());
    (pss, rss)
}

fn parse_rss_and_pss(content: std::str::Lines<'_>) -> (Option<u64>, Option<u64>) {
    #[cfg(feature = "profile-with-tracy")]
    let _span = tracy_client::span!("Process::parse_rss_and_pss");

    let mut rss: Option<u64> = None;
    let mut pss: Option<u64> = None;

    for line in content {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("Rss:") => rss = parts.next().and_then(|v| v.parse().ok()),
            Some("Pss:") => pss = parts.next().and_then(|v| v.parse().ok()),
            _ => continue,
        }
        if rss.is_some() && pss.is_some() {
            break;
        }
    }

    (rss, pss)
}
