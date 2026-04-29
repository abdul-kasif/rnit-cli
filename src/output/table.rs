use crate::core::TableRender;

pub fn print_table<T: TableRender>(entries: &[T]) {
    if entries.is_empty() {
        return;
    }

    let headers = T::headers();
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    for entry in entries {
        let row = entry.row();
        for (i, cell) in row.iter().enumerate() {
            if let Some(w) = widths.get_mut(i) {
                *w = (*w).max(cell.len());
            }
        }
    }

    let header_line = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:<width$}", h, width = widths.get(i).copied().unwrap_or(0)))
        .collect::<Vec<_>>()
        .join("  ");
    println!("{}", header_line);

    let separator = widths
        .iter()
        .map(|w| "-".repeat(*w))
        .collect::<Vec<_>>()
        .join("  ");
    println!("{}", separator);

    for entry in entries {
        let row = entry.row();
        let row_line = row
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
            .join("  ");
        println!("{}", row_line);
    }
}

