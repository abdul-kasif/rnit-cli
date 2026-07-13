use serde::Serialize;

use crate::core::traits::{Alignment, TableRender};

#[derive(Debug, Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: char,
}

impl TableRender for ProcessInfo {
    fn headers() -> &'static [&'static str] {
        &["PID", "NAME", "STATE"]
    }

    fn alignments() -> &'static [Alignment] {
        &[Alignment::Left, Alignment::Left, Alignment::Left]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.state.to_string(),
        ]
    }
}
