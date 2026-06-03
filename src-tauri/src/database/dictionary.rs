use super::schema::Database;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub id: String,
    pub spoken_phrase: String,
    pub replacement: String,
    pub category: Option<String>,
    pub enabled: bool,
    pub use_count: u32,
}

impl Database {
    pub fn insert_dictionary_entry(&self, entry: &DictionaryEntry) -> Result<()> {
        let conn = self.connection();
        conn.execute(
            "INSERT OR REPLACE INTO dictionary (id, spoken_phrase, replacement, category, enabled, use_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                entry.id,
                entry.spoken_phrase,
                entry.replacement,
                entry.category,
                entry.enabled as i32,
                entry.use_count,
            ],
        )?;
        Ok(())
    }

    pub fn get_dictionary_entries(&self) -> Result<Vec<DictionaryEntry>> {
        let conn = self.connection();
        let mut stmt = conn.prepare(
            "SELECT id, spoken_phrase, replacement, category, enabled, use_count
             FROM dictionary ORDER BY spoken_phrase ASC",
        )?;

        let entries = stmt
            .query_map([], |row| {
                Ok(DictionaryEntry {
                    id: row.get(0)?,
                    spoken_phrase: row.get(1)?,
                    replacement: row.get(2)?,
                    category: row.get(3)?,
                    enabled: row.get::<_, i32>(4)? != 0,
                    use_count: row.get(5)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    pub fn get_active_replacements(&self) -> Result<Vec<(String, String)>> {
        let conn = self.connection();
        let mut stmt = conn.prepare(
            "SELECT spoken_phrase, replacement FROM dictionary WHERE enabled = 1",
        )?;

        let replacements = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(replacements)
    }

    pub fn delete_dictionary_entry(&self, id: &str) -> Result<()> {
        let conn = self.connection();
        conn.execute("DELETE FROM dictionary WHERE id = ?1", rusqlite::params![id])?;
        Ok(())
    }

    pub fn increment_dictionary_use(&self, id: &str) -> Result<()> {
        let conn = self.connection();
        conn.execute(
            "UPDATE dictionary SET use_count = use_count + 1 WHERE id = ?1",
            rusqlite::params![id],
        )?;
        Ok(())
    }

    pub fn apply_dictionary(&self, text: &str) -> Result<String> {
        let replacements = self.get_active_replacements()?;
        let mut result = text.to_string();

        for (spoken, replacement) in &replacements {
            let pattern = format!(r"(?i)\b{}\b", regex_lite::escape(spoken));
            if let Ok(re) = regex_lite::Regex::new(&pattern) {
                result = re.replace_all(&result, replacement.as_str()).to_string();
            }
        }

        Ok(result)
    }
}
