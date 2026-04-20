pub fn apply_limit<T>(entries: &mut Vec<T>, limit: Option<usize>) {
    if let Some(lim) = limit {
        if lim == 0 {
            return;
        }
        entries.truncate(lim);
    }
}
