use leptos::*;
use leptos_router::*;
use reqwest;
use serde::{Deserialize, Serialize};
use crate::view::components::display_card::{DisplayCard, CardGrid};

#[derive(Deserialize, Serialize, Clone)]
pub struct SampleTitleAndDescription {
    pub title: String,
    pub description: String,
    pub sanitized_title: String,
}

pub async fn fetch_sample_data() -> Result<Vec<SampleTitleAndDescription>, reqwest::Error> {
    let response = reqwest::get("http://127.0.0.1:8000/api/sample-data")
        .await?;

    let data: Vec<SampleTitleAndDescription> = response.json().await?;
    Ok(data)
}

pub async fn fetch_sample_urls() -> Result<Vec<SampleURL>, reqwest::Error> {
    let response = reqwest::get("http://127.0.0.1:8000/api/sample-urls")
        .await?;

    let data: Vec<SampleURL> = response.json().await?;
    Ok(data)
}

pub async fn fetch_sample_content(url: String) -> Result<String, reqwest::Error> {
    let response = reqwest::get(format!("http://127.0.0.1:8000/api/sample-content/{}", url))
        .await?;

    let data: String = response.text().await?;
    Ok(data)
}

#[component]
pub fn Sample(
    url: String,
) -> impl IntoView {
    let sample_content = create_resource(
        move || url.clone(),
        |url| async move { fetch_sample_content(url).await.unwrap_or_default() }
    );

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            {move || {
                sample_content.get().map(|content| {
                    view! {
                        <div class="sample">
                            {content}
                        </div>
                    }
                })
            }}
        </Suspense>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SampleURL {
    pub url: String,
}

#[component(transparent)]
pub fn SampleRoutes() -> impl IntoView {
    let sample_urls = create_resource(
        || (), |_| async move {
            fetch_sample_urls().await.unwrap_or_default()
        }
    );

    view! {
        {move || {
            sample_urls.get().map(|urls| {
                urls.iter().map(|url| {
                    let url = url.clone();
                    view! {
                        <Route
                            path=format!("/sample/{}", url.url)
                            view=move || {
                                view! { <Sample url=url.url.clone() /> }
                            }
                        />
                    }
                }).collect::<Vec<_>>()
            }).unwrap_or_default()
        }}
            
    }
}
      
                    

#[component]
pub fn SampleCards() -> impl IntoView {
    let sample_data = create_resource(
        || (), |_| async move {
            fetch_sample_data().await.unwrap_or_default()
        }
    );

    view! {
        <Suspense>
            <div class="sample-items">
                <CardGrid colums=3>
                    {move || sample_data.get().map(|data| {
                        data.into_iter().map(|sample| {
                            view! {
                                <DisplayCard
                                    link= format!("/portfolio/{}", sample.sanitized_title)
                                    link_is_external=false
                                    link_title=sample.title.clone()
                                >
                                    <h3>{sample.title}</h3>
                                    <p>{sample.description}</p>
                                </DisplayCard>
                            }
                        }).collect::<Vec<_>>()
                    }).unwrap_or_default()}
                </CardGrid>
            </div>
        </Suspense>
    }
}


