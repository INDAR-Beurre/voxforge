use super::schema::Database;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionRecord {
    pub id: String,
    pub text: String,
    pub timestamp: String,
    pub duration_ms: u64,
    pub word_count: u32,
    pub mode: String,
    pub provider: String,
    pub model_name: Option<String>,
    pub language: Option<String>,
    pub target_app: Option<String>,
    pub is_favorite: bool,
}

impl Database {
    pub fn insert_transcription(&self, record: &TranscriptionRecord) -> Result<()> {
        let conn = self.connection();
        conn.execute(
            "INSERT INTO transcriptions (id, text, timestamp, duration_ms, word_count, mode, provider, model_name, language, target_app, is_favorite)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                record.id,
                record.text,
                record.timestamp,
                record.duration_ms,
                record.word_count,
                record.mode,
                record.provider,
                record.model_name,
                record.language,
                record.target_app,
                record.is_favorite as i32,
            ],
        )?;
        Ok(())
    }

    pub fn get_transcriptions(&self, limit: u32, offset: u32) -> Result<Vec<TranscriptionRecord>> {
        let conn = self.connection();
        let mut stmt = conn.prepare(
            "SELECT id, text, timestamp, duration_ms, word_count, mode, provider, model_name, language, target_app, is_favorite
             FROM transcriptions ORDER BY timestamp DESC LIMIT ?1 OFFSET ?2",
        )?;

        let records = stmt
            .query_map(rusqlite::params![limit, offset], |row| {
                Ok(TranscriptionRecord {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    timestamp: row.get(2)?,
                    duration_ms: row.get(3)?,
                    word_count: row.get(4)?,
                    mode: row.get(5)?,
                    provider: row.get(6)?,
                    model_name: row.get(7)?,
                    language: row.get(8)?,
                    target_app: row.get(9)?,
                    is_favorite: row.get::<_, i32>(10)? != 0,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(records)
    }

    pub fn search_transcriptions(&self, query: &str, limit: u32) -> Result<Vec<TranscriptionRecord>> {
        let conn = self.connection();
        let mut stmt = conn.prepare(
            "SELECT id, text, timestamp, duration_ms, word_count, mode, provider, model_name, language, target_app, is_favorite
             FROM transcriptions WHERE text LIKE ?1 ORDER BY timestamp DESC LIMIT ?2",
        )?;

        let pattern = format!("%{}%", query);
        let records = stmt
            .query_map(rusqlite::params![pattern, limit], |row| {
                Ok(TranscriptionRecord {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    timestamp: row.get(2)?,
                    duration_ms: row.get(3)?,
                    word_count: row.get(4)?,
                    mode: row.get(5)?,
                    provider: row.get(6)?,
                    model_name: row.get(7)?,
                    language: row.get(8)?,
                    target_app: row.get(9)?,
                    is_favorite: row.get::<_, i32>(10)? != 0,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(records)
    }

    pub fn toggle_favorite(&self, id: &str) -> Result<bool> {
        let conn = self.connection();
        conn.execute(
            "UPDATE transcriptions SET is_favorite = CASE WHEN is_favorite = 0 THEN 1 ELSE 0 END WHERE id = ?1",
            rusqlite::params![id],
        )?;

        let is_fav: bool = conn.query_row(
            "SELECT is_favorite FROM transcriptions WHERE id = ?1",
            rusqlite::params![id],
            |row| row.get::<_, i32>(0).map(|v| v != 0),
        )?;

        Ok(is_fav)
    }

    pub fn delete_transcription(&self, id: &str) -> Result<()> {
        let conn = self.connection();
        conn.execute("DELETE FROM transcriptions WHERE id = ?1", rusqlite::params![id])?;
        Ok(())
    }

    pub fn get_transcription_count(&self) -> Result<u64> {
        let conn = self.connection();
        let count: u64 = conn.query_row(
            "SELECT COUNT(*) FROM transcriptions",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}
