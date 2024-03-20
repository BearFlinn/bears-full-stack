use leptos::*;
use crate::view::components::WritingForm;


#[component]
pub fn Playground() -> impl IntoView {
    view! {
        <div class="playground">
            <h1>"Playground"</h1>
            <WritingForm />
        </div>
    }
}