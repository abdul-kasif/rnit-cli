pub fn apply_sort<T, F>(entries: &mut [T], sort_fn: Option<F>)
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    if let Some(comparator) = sort_fn {
        entries.sort_by(comparator);
    }
}
