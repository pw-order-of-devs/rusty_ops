use async_graphql::{parser::types::Field, ContextBase, Positioned, SelectionField};
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

pub fn selected_fields<'a>(
    ctx: &'a ContextBase<'_, &Positioned<Field>>,
) -> Vec<SelectionField<'a>> {
    match ctx.look_ahead().selection_fields().first() {
        Some(item) => match item.name() {
            "get" => item
                .selection_set()
                .find(|f| f.name() == "entries")
                .unwrap()
                .selection_set()
                .collect(),
            "getById" => item.selection_set().collect(),
            _ => vec![],
        },
        _ => vec![],
    }
}
