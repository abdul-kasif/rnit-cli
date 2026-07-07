pub enum Alignment {
    Left,
    Right,
}

pub trait TableRender {
    fn headers() -> &'static [&'static str];

    fn alignments() -> &'static [Alignment];

    fn row(&self) -> Vec<String>;
}
