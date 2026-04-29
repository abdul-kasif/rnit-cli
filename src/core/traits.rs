pub trait TableRender {
    fn headers() -> &'static [&'static str];
    fn row(&self) -> Vec<String>;
}
