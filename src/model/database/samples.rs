use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Sample {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub description: String,
}

pub fn insert_sample(conn: &Connection, title: &str, content: &str, description: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO samples (title, content, description) VALUES (?1, ?2, ?3)",
        params![title, content, description],
    )?;
    Ok(())
}

pub fn get_all_samples(conn: &Connection) -> Result<Vec<Sample>> {
    let mut stmt = conn.prepare("SELECT id, title, content FROM samples")?;
    let samples = stmt
        .query_map([], |row| {
            Ok(Sample {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                description: row.get(3)?,
            })
        })?
        .map(|row| row.unwrap())
        .collect();
    Ok(samples)
}

pub fn get_sample_by_id(conn: &Connection, id: i32) -> Result<Sample> {
    let mut stmt = conn.prepare("SELECT id, title, content FROM samples WHERE id = ?1")?;
    let sample = stmt
        .query_row(params![id], |row| {
            Ok(Sample {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                description: row.get(3)?,
            })
        })?;
    Ok(sample)
}

#[derive(Serialize, Deserialize, Clone)]
struct TitleResponse {
    titles: Vec<String>,
}

pub fn get_all_sample_titles(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT title FROM samples")?;
    let titles = stmt
        .query_map([], |row| {
            Ok(row.get(0)?)
        })?
        .map(|row| row.unwrap())
        .collect();
    Ok(titles)
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