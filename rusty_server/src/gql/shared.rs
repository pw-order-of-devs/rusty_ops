use domain::commons::search::SearchOptions;
use domain::RustyDomainItem;

pub fn paginate<T: RustyDomainItem>(
    entries: &[T],
    options: Option<SearchOptions>,
) -> (usize, usize, usize, Vec<T>) {
    let options = options.unwrap_or_default();
    let mut page = options.page_number.unwrap_or(1) as usize;
    if page == 0 {
        page = 1
    };
    let page_size = options.page_size.unwrap_or(20) as usize;
    let paginated = entries
        .iter()
        .skip((page - 1) * page_size)
        .take(page_size)
        .cloned()
        .collect::<Vec<T>>();
    (entries.len(), page, page_size, paginated)
}
