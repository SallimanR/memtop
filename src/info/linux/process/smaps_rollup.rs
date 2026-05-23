use std::{fs::File, io::Read};

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
