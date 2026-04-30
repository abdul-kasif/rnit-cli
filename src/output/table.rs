use crate::core::TableRender;

pub fn print_table<T: TableRender>(entries: &[T]) {
    let headers: Vec<String> = T::headers().iter().map(|h| h.to_string()).collect();

    if headers.is_empty() {
        return;
    }

    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    for entry in entries {
        for (i, cell) in entry.row().iter().enumerate() {
            if let Some(w) = widths.get_mut(i) {
                *w = (*w).max(cell.len());
            }
        }
    }

    let format_line = |cells: &[String]| -> String {
        cells
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                format!(
                    "{:<width$}",
                    cell,
                    width = widths.get(i).copied().unwrap_or(0)
                )
            })
            .collect::<Vec<_>>()
            .join("  ")
    };

    println!("{}", format_line(&headers));
    println!(
        "{}",
        widths
            .iter()
            .map(|w| "-".repeat(*w))
            .collect::<Vec<_>>()
            .join("  ")
    );

    if entries.is_empty() {
        eprintln!("(no results)");
    } else {
        for entry in entries {
            println!("{}", format_line(&entry.row()));
        }
    }
}

