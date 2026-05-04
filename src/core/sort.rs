use crate::core::SortOrder;

pub fn apply_sort<T, F>(entries: &mut [T], sort_fn: Option<F>, order: SortOrder)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    if let Some(mut compare) = sort_fn {
        entries.sort_by(|a, b| compare(a, b));

        if order == SortOrder::Desc {
            entries.reverse();
        }
    }
}
