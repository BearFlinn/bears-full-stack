use leptos::*;
use crate::view::components::{Carousel, Slide};


#[component]
pub fn Playground() -> impl IntoView {
    view! {
        <div class="playground">
            <h1>"Playground"</h1>
            <Carousel>
                <Slide id=1>"Slide 1"</Slide>
                <Slide id=2>"Slide 2"</Slide>
                <Slide id=3>"Slide 3"</Slide>
            </Carousel>
        </div>
    }
}