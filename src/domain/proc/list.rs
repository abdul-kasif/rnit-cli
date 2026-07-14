use std::{fs, path::Path};

use crate::domain::proc::ProcessInfo;

pub fn get_all_processes() -> Vec<ProcessInfo> {
    let mut processes = Vec::new();
    let proc_dir = Path::new("/proc");

    if let Ok(entries) = fs::read_dir(proc_dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name_str = file_name.to_string_lossy();

            if let Ok(pid) = name_str.parse::<u32>() {
                if let Some(info) = parse_process_stat(pid) {
                    processes.push(info);
                }
            }
        }
    }
    processes
}

fn parse_process_stat(pid: u32) -> Option<ProcessInfo> {
    let stat_path = format!("/proc/{}/stat", pid);

    let stat_content = fs::read_to_string(stat_path).ok()?;

    let parts: Vec<&str> = stat_content.split_whitespace().collect();
    if parts.len() > 2 {
        let name = parts[1].trim_matches(|c| c == '(' || c == ')').to_string();
        let state = parts[2].chars().next().unwrap_or('?');

        return Some(ProcessInfo { pid, name, state });
    }
    None
}
