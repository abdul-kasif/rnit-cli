pub fn format_human_readable_size(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let units_len = units.len();
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units_len - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size.trunc() as u64, units[unit_index])
    } else {
        format!("{:.1} {}", size, units[unit_index])
    }
}
