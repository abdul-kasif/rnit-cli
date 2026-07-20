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

    parse_stat_content(pid, content_buf, page_size)
}

fn parse_stat_content(pid: u32, content: &str, page_size: u64) -> Option<ProcessInfo> {
    let start_paren = content.find('(')?;
    let end_paren = content.rfind(')')?;

    let name = content[start_paren + 1..end_paren].to_string();

    let remainder = content[end_paren + 1..].trim_start();
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_stat_string(name: &str, state: char, rss_pages: u64) -> String {
        // Creates a string with the exact number of fields needed to reach RSS.
        // 1: pid
        // 2: name
        // 3: state
        // 4-23: zeros (20 filler fields)
        // 24: rss
        format!(
            "1234 ({}) {} 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 {} 0 0",
            name, state, rss_pages
        )
    }

    #[test]
    fn test_parse_standard_process() {
        let content = create_stat_string("systemd", 'S', 500);
        let info = parse_stat_content(1234, &content, 4096).unwrap();

        assert_eq!(info.pid, 1234);
        assert_eq!(info.name, "systemd");
        assert_eq!(info.state, 'S');
        assert_eq!(info.rss, 500 * 4096);
    }

    #[test]
    fn test_parse_process_name_with_spaces() {
        // Tricky edge case: Thread names often contain spaces
        let content = create_stat_string("kworker/u4:2", 'I', 0);
        let info = parse_stat_content(1234, &content, 4096).unwrap();

        assert_eq!(info.name, "kworker/u4:2");
        assert_eq!(info.state, 'I');
    }

    #[test]
    fn test_parse_process_name_with_parentheses() {
        // Crucial edge case: Processes can rename themselves to contain parentheses.
        let content = create_stat_string("worker (1)", 'R', 100);
        let info = parse_stat_content(1234, &content, 1024).unwrap();

        assert_eq!(info.name, "worker (1)");
        assert_eq!(info.state, 'R');
        assert_eq!(info.rss, 100 * 1024);
    }

    #[test]
    fn test_parse_missing_rss_field() {
        // Simulates a truncated file where the RSS field hasn't been written
        let content = "1234 (short_proc) S 0 0";
        let info = parse_stat_content(1234, content, 4096).unwrap();

        // Should gracefully fall back to 0 due to `.unwrap_or(0)`
        assert_eq!(info.rss, 0);
    }

    #[test]
    fn test_parse_malformed_no_parentheses() {
        // Completely invalid format, should return None
        let content = "1234 no_parens_here S 0 0 0";
        assert!(parse_stat_content(1234, content, 4096).is_none());
    }

    #[test]
    fn test_parse_zero_rss() {
        // Kernel threads often have 0 RSS
        let content = create_stat_string("kthreadd", 'S', 0);
        let info = parse_stat_content(2, &content, 4096).unwrap();

        assert_eq!(info.rss, 0);
    }

    #[test]
    fn test_integration_current_process() {
        // This test actually reads the real filesystem.
        // It ensures the whole pipeline works by finding the Rust test runner process.
        if cfg!(target_os = "linux") {
            let processes = get_all_processes();
            let my_pid = std::process::id();

            let my_proc = processes.iter().find(|p| p.pid == my_pid);

            assert!(
                my_proc.is_some(),
                "The test runner PID ({}) should be found in /proc",
                my_pid
            );

            let proc_info = my_proc.unwrap();
            assert!(proc_info.rss > 0, "Test runner should consume some memory");
        }
    }
}
