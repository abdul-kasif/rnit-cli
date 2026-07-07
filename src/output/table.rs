use crate::core::traits::{Alignment, TableRender};

pub fn print_table<T: TableRender>(entries: &[T]) {
    let headers: Vec<String> = T::headers().iter().map(|h| h.to_string()).collect();

    if headers.is_empty() {
        return;
    }

    let alignments = T::alignments();
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
                let width = widths.get(i).copied().unwrap_or(0);
                let align = alignments.get(i).unwrap_or(&Alignment::Left);

                match align {
                    Alignment::Left => format!("{:<width$}", cell, width = width),
                    Alignment::Right => format!("{:>width$}", cell, width = width),
                }
            })
            .collect::<Vec<_>>()
            .join("  ")
    };

    println!("{}", format_line(&headers));

    let separators: Vec<String> = widths
        .iter()
        .enumerate()
        .map(|(i, &w)| {
            let dashes = "-".repeat(w);
            let align = alignments.get(i).unwrap_or(&Alignment::Left);
            match align {
                Alignment::Left => format!("{:<width$}", dashes, width = w),
                Alignment::Right => format!("{:>width$}", dashes, width = w),
            }
        })
        .collect();

    println!("{}", separators.join("  "));

    if entries.is_empty() {
        eprintln!("(no results)");
    } else {
        for entry in entries {
            println!("{}", format_line(&entry.row()));
        }
    }
}
