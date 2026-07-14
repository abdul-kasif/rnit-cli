use std::{fmt::Write, fs, io::Read, path::Path};

use crate::domain::proc::ProcessInfo;

fn get_system_page_size() -> u64 {
    unsafe {
        let res = libc::sysconf(libc::_SC_PAGESIZE);
        if res > 0 { res as u64 } else { 4096 }
    }
}

pub fn get_all_processes() -> Vec<ProcessInfo> {
    let proc_dir = Path::new("/proc");

    let Ok(entries) = fs::read_dir(proc_dir) else {
        return Vec::new();
    };

    let page_size = get_system_page_size();
    let mut path_buf = String::with_capacity(64);
    let mut content_buf = String::with_capacity(1024);

    entries
        .flatten()
        .filter_map(|entry| {
            let file_name = entry.file_name();
            let name_str = file_name.to_string_lossy();

            let pid = name_str.parse::<u32>().ok()?;

            parse_process_stat(pid, &mut path_buf, &mut content_buf, page_size)
        })
        .collect()
}

fn parse_process_stat(
    pid: u32,
    path_buf: &mut String,
    content_buf: &mut String,
    page_size: u64,
) -> Option<ProcessInfo> {
    path_buf.clear();
    write!(path_buf, "/proc/{}/stat", pid).ok()?;

    content_buf.clear();
    let mut file = fs::File::open(&path_buf).ok()?;
    file.read_to_string(content_buf).ok()?;

    let start_paren = content_buf.find('(')?;
    let end_paren = content_buf.rfind(')')?;

    let name = content_buf[start_paren + 1..end_paren].to_string();

    let remainder = content_buf[end_paren + 1..].trim_start();
    let state = remainder.chars().next().unwrap_or('?');

    let mut fields = remainder.split_whitespace();
    let rss_pages = fields
        .nth(21)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let rss_bytes = rss_pages * page_size;

    Some(ProcessInfo {
        pid,
        name,
        state,
        rss: rss_bytes,
    })
}
