use leptos::*;

use crate::view::components::sample_items::SampleCards;

#[component]
pub fn Portfolio() -> impl IntoView {
    view! {
        <div class="portfolio">
            <h1>"Portfolio"</h1>
            <SampleCards />
        </div>
    }
}