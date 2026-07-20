use clap::ValueEnum;
use serde::Serialize;

use crate::core::{
    traits::{Alignment, TableRender},
    utils::format_human_readable_size,
};

#[derive(Debug, Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: char,
    pub rss: u64,
}

impl TableRender for ProcessInfo {
    fn headers() -> &'static [&'static str] {
        &["PID", "NAME", "STATE", "MEMORY"]
    }

    fn alignments() -> &'static [Alignment] {
        &[
            Alignment::Left,
            Alignment::Left,
            Alignment::Left,
            Alignment::Right,
        ]
    }

    fn row(&self) -> Vec<String> {
        let rss = format_human_readable_size(self.rss);
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.state.to_string(),
            rss,
        ]
    }
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Default)]
pub enum ProcSortField {
    #[default]
    Name,
    Size,
}
