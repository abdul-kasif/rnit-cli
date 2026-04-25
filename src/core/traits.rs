pub trait TableRender {
    fn headers() -> Vec<&'static str>;
    fn row(&self) -> Vec<String>;
}
