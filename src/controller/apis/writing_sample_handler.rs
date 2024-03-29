use axum::{
    extract::{Multipart, Path}, http::StatusCode, response::IntoResponse, Json
};

use std::io::{BufReader, Read};
use pulldown_cmark::Parser;

use crate::model::{connection, samples};

fn md_to_html(md_content: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);

    let parser = Parser::new_ext(md_content, options);
    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    html_buf
}

pub async fn submit_form(mut multipart: Multipart) -> impl IntoResponse {
    let mut title = String::new();
    let mut description = String::new();
    let mut file_content = String::new();
    let mut file_name = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "title" => {
                let data = field.bytes().await.unwrap();
                title = String::from_utf8(data.to_vec()).unwrap();
            }
            "description" => {
                let data = field.bytes().await.unwrap();
                description = String::from_utf8(data.to_vec()).unwrap();
            }
            "content" => {
                file_name = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap();
                let mut reader = BufReader::new(data.as_ref());
                reader.read_to_string(&mut file_content).unwrap();
            }
            _ => {}
        }
    }

    if title.is_empty() || description.is_empty() {
        return (StatusCode::BAD_REQUEST, "Title and description are required.").into_response();
    }

    let content = if file_name.ends_with(".md") {
        md_to_html(&file_content)
    } else {
        file_content
    };

    let conn = match connection::connect() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    match samples::insert_sample(&conn, &title, &content, &description) {
        Ok(()) => {
            // Insertion successful
            (StatusCode::OK, "Form submitted successfully!").into_response()
        }
        Err(err) => {
            eprintln!("Failed to insert sample into the database: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    };

    // Placeholder response
    (StatusCode::OK, "Form submitted successfully!").into_response()
}

pub async fn get_sample_data() -> impl IntoResponse {
    let conn = match connection::connect() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };
    let result = samples::get_sample_titles_and_descriptions(&conn);
    match result {
        Ok(samples) => (StatusCode::OK, Json(samples)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
    }
}

pub async fn get_sample_content(
    Path(url): Path<String>,
) -> impl IntoResponse {
    let conn = match connection::connect() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };
    let result = samples::get_sample_content_from_url(&conn, &url);
    match result {
        Ok(samples) => (StatusCode::OK, Json(samples)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
    }
}

pub async fn get_sample_url() -> impl IntoResponse {
    let conn = match connection::connect() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };
    let result = samples::get_sample_urls(&conn);
    match result {
        Ok(samples) => (StatusCode::OK, Json(samples)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
    }
}