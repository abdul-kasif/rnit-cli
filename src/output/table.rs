use crate::domain::fs::FileEntry;

pub fn print_table(entries: &[FileEntry]) {
    let max_name_len = entries.iter().map(|e| e.name.len()).max().unwrap_or(4);

    let name_header = "NAME";
    let size_header = "SIZE";
    let type_header = "TYPE";

    println!(
        "{:<width$} {:>9}  {}",
        name_header,
        size_header,
        type_header,
        width = max_name_len
    );

    println!(
        "{:-<width$} {:->9}  {:->4}",
        "",
        "",
        "",
        width = max_name_len
    );

    for entry in entries {
        let type_str = if entry.is_dir { "DIR" } else { "FILE" };

        let size_str = if entry.is_dir {
            "-".to_string()
        } else {
            format!("{}B", entry.size)
        };

        println!(
            "{:<width$} {:>9}  {}",
            entry.name,
            size_str,
            type_str,
            width = max_name_len
        );
    }
}
