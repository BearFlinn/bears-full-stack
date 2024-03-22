use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use crate::model::database::utils::sanitize_title;

#[derive(Debug)]
pub struct Sample {
    pub id: i32,
    pub title: String,
    pub sanitized_title: String,
    pub content: String,
    pub description: String,
}

pub fn insert_sample(conn: &Connection, title: &str, content: &str, description: &str) -> Result<()> {
    let sanitized_title = sanitize_title(title);
    conn.execute(
        "INSERT INTO samples (title, sanitized_title, content, description) VALUES (?1, ?2, ?3, ?4)",
        params![title, sanitized_title, content, description],
    )?;
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SampleURL {
    pub url: String,
}

pub fn get_sample_urls(conn: &Connection) -> Result<Vec<SampleURL>> {
    let mut stmt = conn.prepare("SELECT sanitized_title FROM samples")?;
    let sample_url = stmt
    .query_map(params![], |row| {
        let url: String = row.get(0)?;
        Ok(SampleURL {
            url,
        })
    })?
    .collect::<Result<Vec<SampleURL>>>()?;
    Ok(sample_url)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SampleContent {
    pub content: String,
}


pub fn get_sample_content_from_url(conn: &Connection, url: &str) -> Result<SampleContent> {
    let mut stmt = conn.prepare(format!("SELECT content FROM samples WHERE sanitized_title = '{}'", url).as_str())?;
    let sample_content = stmt
    .query_row([], |row| {
        let content: String = row.get(0)?;
        Ok(SampleContent {
            content,
        })
    })?;
    Ok(sample_content)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SampleTitleAndDescription {
    pub title: String,
    pub description: String,
    pub sanitized_title: String,
}

pub fn get_sample_titles_and_descriptions(conn: &Connection) -> Result<Vec<SampleTitleAndDescription>> {
    let mut stmt = conn.prepare("SELECT title, sanitized_title, description FROM samples")?;
    let results = stmt
        .query_map([], |row| {
            let title: String = row.get(0)?;
            let sanitized_title = row.get(1)?;
            let description: String = row.get(2)?;
            Ok(SampleTitleAndDescription {
                title,
                description,
                sanitized_title,
            })
        })?
        .collect::<Result<Vec<SampleTitleAndDescription>>>()?;
    Ok(results)
}

pub fn update_sample(conn: &Connection, id: i32, title: &str, content: &str) -> Result<()> {
    conn.execute(
        "UPDATE samples SET title = ?1, content = ?2 WHERE id = ?3",
        params![title, content, id],
    )?;
    Ok(())
}

pub fn delete_sample(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM samples WHERE id = ?1", params![id])?;
    Ok(())
}