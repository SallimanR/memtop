pub mod process_list;
pub mod process_tree;

use std::{
    fs::{self, DirEntry, File, ReadDir},
    io::{self, Read},
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

#[derive(Debug, Default)]
pub enum ProcessType {
    #[default]
    Regular,
    Kernel,
    Thread,
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

pub fn proc_get_threads_ids(
    pid: u32,
) -> Option<FilterMap<Flatten<ReadDir>, impl FnMut(DirEntry) -> Option<u32>>> {
    let files = fs::read_dir(format!("/proc/{}/task", pid)).ok()?;
    Some(files.flatten().filter_map(|entry| {
        let name = entry.file_name();
        let s = name.to_str()?;
        s.parse::<u32>().ok()
    }))
}

pub fn proc_get_stat(pid: u32, buf: &mut [u8; 512]) -> Option<ProcessStat> {
    let file_path = format!("/proc/{}/stat", pid);
    file_get_stat(file_path, buf)
}

pub fn thread_get_stat(pid: u32, thread_id: u32, buf: &mut [u8; 512]) -> Option<ProcessStat> {
    let file_path = format!("/proc/{}/task/{}/stat", pid, thread_id);
    file_get_stat(file_path, buf)
}

pub fn proc_get_tgid(pid: u32, buf: &mut [u8; 512]) -> Option<u32> {
    let file_path = format!("/proc/{}/status", pid);
    file_get_tgid(file_path, buf)
}

pub fn proc_get_name(pid: u32) -> String {
    let file_path = format!("/proc/{}/comm", pid);
    file_get_name(file_path)
}

pub fn thread_get_name(pid: u32, thread_id: u32) -> String {
    let file_path = format!("/proc/{}/task/{}/comm", pid, thread_id);
    file_get_name(file_path)
}

/// A wrapper around the data in `/proc/<PID>/stat`. For documentation, see:
/// - <https://manpages.ubuntu.com/manpages/noble/man5/proc_pid_stat.5.html>
/// - <https://man7.org/linux/man-pages/man5/proc_pid_status.5.html>
///
/// Note this does not necessarily get all fields, only the ones we use
pub(crate) struct ProcessStat {
    /// The filename of the executable without parentheses.
    pub comm: String,

    /// The current process state, represented by a char.
    pub state: char,

    /// The parent process PID.
    pub ppid: u32,

    /// Kernel thread
    pub is_kernel_thread: bool,

    /// The amount of time this process has been scheduled in user mode in clock
    /// ticks.
    pub utime: u64,

    /// The amount of time this process has been scheduled in kernel mode in
    /// clock ticks.
    pub stime: u64,

    /// The resident set size, or the number of pages the process has in real
    /// memory.
    rss: u64,

    /// The virtual memory size in bytes.
    pub vsize: u64,

    /// The start time of the process, represented in clock ticks.
    pub start_time: u64,

    /// The kernel scheduling priority.
    pub priority: i32,

    /// The nice value (user-settable scheduling hint).
    #[cfg(unix)]
    pub nice: i32,
}

pub fn file_get_stat(file_path: String, buf: &mut [u8; 512]) -> Option<ProcessStat> {
    let mut f = File::open(file_path).ok()?;

    let n = f.read(&mut buf[..]).ok()?;
    let content = str::from_utf8(&buf[..n]).ok()?;

    let (comm, fields) = {
        let start_paren = content.find('(').ok_or("start paren missing").ok()?;
        let end_paren = content.find(')').ok_or("end paren missing").ok()?;

        (
            content[start_paren + 1..end_paren].to_string(),
            &content[end_paren + 2..],
        )
    };
    let mut fields = fields.split_whitespace();

    let state = fields.next()?.chars().next().ok_or("missing state").ok()?;
    let ppid = fields.next()?.parse().ok()?;

    // Skip: pgrp, session, tty_nr, tpgid
    let mut fields = fields.skip(4);

    let flags: u32 = fields.next()?.parse().ok()?;
    let is_kernel = is_kernel_thread(flags);

    let mut fields = fields.skip(4);

    let utime: u64 = fields.next()?.parse().ok()?;
    let stime: u64 = fields.next()?.parse().ok()?;
    let _cutime: i32 = fields.next()?.parse().ok()?;
    let _cstime: i32 = fields.next()?.parse().ok()?;
    let priority: i32 = fields.next()?.parse().ok()?;
    let nice: i32 = fields.next()?.parse().ok()?;
    let _num_threads: i32 = fields.next()?.parse().ok()?;
    let _itrealvalue: i32 = fields.next()?.parse().ok()?;
    let start_time: u64 = fields.next()?.parse().ok()?;
    let vsize: u64 = fields.next()?.parse().ok()?;
    let rss: u64 = fields.next()?.parse().ok()?;

    Some(ProcessStat {
        comm,
        state,
        ppid,
        is_kernel_thread: is_kernel,
        utime,
        stime,
        rss,
        vsize,
        start_time,
        priority,
        nice,
    })
}

fn is_kernel_thread(flags: u32) -> bool {
    const PF_KTHREAD: u32 = 0x00200000;
    flags & PF_KTHREAD != 0
}

pub fn file_get_tgid(file_path: String, buf: &mut [u8; 512]) -> Option<u32> {
    let mut f = File::open(file_path).ok()?;

    let n = f.read(&mut buf[..]).ok()?;
    let content = str::from_utf8(&buf[..n]).ok()?;

    for line in content.lines() {
        if line.starts_with("Tgid:") {
            return line.split_whitespace().nth(1)?.parse().ok();
        }
    }
    None
}

fn file_get_name(file_path: String) -> String {
    #[cfg(feature = "profile-with-tracy")]
    let _span = tracy_client::span!("process::file_get_name");

    let content = fs::read_to_string(file_path);
    match content {
        Ok(info) => info,
        Err(_) => "".to_owned(),
    }
}

pub fn proc_get_smaps_rollup(pid: u32, buf: &mut [u8]) -> (Option<u64>, Option<u64>) {
    #[cfg(feature = "profile-with-tracy")]
    let _span = tracy_client::span!("Process::get_smaps_rollup");

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
