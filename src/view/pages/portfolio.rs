use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone)]
struct TitleResponse {
    titles: Vec<String>,
}

impl IntoIterator for TitleResponse {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.titles.into_iter()
    }
}

#[component(transparent)]
pub fn WritingSampleRoutes() -> impl IntoView {
    let titles = create_resource(
        || (), |_| async move {
            let response = reqwest::get("http://localhost:8000/api/sample-titles").await.unwrap();
            response.json::<TitleResponse>().await.unwrap()
        }
    );
    view! {
        {move || titles.get().map(|titles| {
            titles.into_iter().map(|title| {
                view! {
                    <Route path={format!("/{}", title)} view=move || view! {  <Portfolio /> }/>
                }
            }).collect::<Vec<_>>()
        }).unwrap_or_default()
    }}
}

#[component]
pub fn Portfolio() -> impl IntoView {
    view! {
        <div class="portfolio">
            <h1>"Portfolio"</h1>
        </div>
    }
}