pub fn apply_sort<T, F>(entries: &mut [T], sort_fn: Option<F>)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    if let Some(mut compare) = sort_fn {
        entries.sort_by(|a, b| compare(a, b));
    }
}

