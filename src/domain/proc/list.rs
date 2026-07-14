use std::{fmt::Write, fs, io::Read, path::Path};

use crate::domain::proc::ProcessInfo;

pub fn get_all_processes() -> Vec<ProcessInfo> {
    let proc_dir = Path::new("/proc");

    let Ok(entries) = fs::read_dir(proc_dir) else {
        return Vec::new();
    };

    let mut path_buf = String::with_capacity(64);
    let mut content_buf = String::with_capacity(1024);

    entries
        .flatten()
        .filter_map(|entry| {
            let file_name = entry.file_name();
            let name_str = file_name.to_string_lossy();

            let pid = name_str.parse::<u32>().ok()?;

            parse_process_stat(pid, &mut path_buf, &mut content_buf)
        })
        .collect()
}

fn parse_process_stat(
    pid: u32,
    path_buf: &mut String,
    content_buf: &mut String,
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

    Some(ProcessInfo { pid, name, state })
}
