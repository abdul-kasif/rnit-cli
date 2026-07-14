use std::{fs, path::Path};

use crate::domain::proc::ProcessInfo;

pub fn get_all_processes() -> Vec<ProcessInfo> {
    let proc_dir = Path::new("/proc");

    let Ok(entries) = fs::read_dir(proc_dir) else {
        return Vec::new();
    };

    entries
        .flatten()
        .filter_map(|entry| {
            let file_name = entry.file_name();
            let name_str = file_name.to_string_lossy();

            let pid = name_str.parse::<u32>().ok()?;

            parse_process_stat(pid)
        })
        .collect()
}

fn parse_process_stat(pid: u32) -> Option<ProcessInfo> {
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content = fs::read_to_string(stat_path).ok()?;

    let start_paren = stat_content.find('(')?;
    let end_paren = stat_content.rfind(')')?;

    let name = stat_content[start_paren + 1..end_paren].to_string();

    let remainder = stat_content[end_paren + 1..].trim_start();

    let state = remainder.chars().next().unwrap_or('?');

    Some(ProcessInfo { pid, name, state })
}
