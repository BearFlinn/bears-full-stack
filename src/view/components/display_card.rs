use leptos::*;

// Simple dynamic card, that takes children. Can accept an optional link and will render a clickable card (separate css) if link is provided
#[component]
pub fn DisplayCard(
    #[prop(optional)] link: String,
    #[prop(default = false)] link_is_external: bool,
    #[prop(optional)] link_title: Option<String>,
    children: Children
) -> impl IntoView {
    let link_atr = link.clone();
    let title = link_title.clone();
    // If no link is provided, render the card as a static card
    if link.is_empty() {
        view! {
            <div class="static-card">
                <div class="card-content">
                   || {children()}
                </div>
            </div>
        };
    } else {
        // Check if link is external, if so, set target to _blank. (Opens in new tab)
        // Otherwise, set target to _self. (Stays in same tab, used for site navigation)
        let target_attr = if link_is_external { "_blank" } else { "_self" };
        // Render the card as a clickable card if a link is provided.
        view! {
            <a class="clickable-card" title=link_title href=link target=target_attr>
                <div class="card-content">
                   || {children()}
                </div>
            </a>
        };
    }
    // Check if link is provided, if so, check if title is provided. If not, throw an error
    if link_atr.is_empty() && title.is_none() {
        logging::log!("DisplayCard: title is required if link is provided.");
    }
}

/// A grid of cards that will auto-resize based on the number of columns.
/// The `colums` prop is the number of columns the grid should have.
#[component]
pub fn CardGrid(
    children: Children,
    /// The number of columns the grid should have.
    colums: usize
) -> impl IntoView {
    let col_count = colums;
    view! {
        <div class="card-grid">
            <div class="grid"  
                 // Set the grid to have a template of `colums` number of columns, with each column
                 // having a min width of 0 and a max width of 1fr (takes up as much space as it can).
                 style=format!("grid-template-columns: repeat({}, minmax(0, 1fr));", col_count)>
                // Render the children inside of the grid
                || {children()}
            </div>
        </div>
    }
}
