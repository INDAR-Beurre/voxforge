use super::schema::Database;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub total_words: u64,
    pub total_duration_ms: u64,
    pub total_sessions: u64,
    pub total_transcriptions: u64,
    pub average_wpm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub total_words: u64,
    pub total_duration_ms: u64,
    pub session_count: u64,
    pub transcription_count: u64,
}

impl Database {
    pub fn record_transcription_stats(&self, word_count: u32, duration_ms: u64) -> Result<()> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let conn = self.connection();

        conn.execute(
            "INSERT INTO stats (id, date, total_words, total_duration_ms, session_count, transcription_count)
             VALUES (?1, ?2, ?3, ?4, 0, 1)
             ON CONFLICT(date) DO UPDATE SET
                total_words = total_words + ?3,
                total_duration_ms = total_duration_ms + ?4,
                transcription_count = transcription_count + 1",
            rusqlite::params![
                uuid::Uuid::new_v4().to_string(),
                today,
                word_count,
                duration_ms,
            ],
        )?;
        Ok(())
    }

    pub fn record_session(&self) -> Result<()> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let conn = self.connection();

        conn.execute(
            "INSERT INTO stats (id, date, total_words, total_duration_ms, session_count, transcription_count)
             VALUES (?1, ?2, 0, 0, 1, 0)
             ON CONFLICT(date) DO UPDATE SET session_count = session_count + 1",
            rusqlite::params![uuid::Uuid::new_v4().to_string(), today],
        )?;
        Ok(())
    }

    pub fn get_overall_stats(&self) -> Result<UsageStats> {
        let conn = self.connection();
        let (total_words, total_duration_ms, total_sessions, total_transcriptions): (u64, u64, u64, u64) = conn.query_row(
            "SELECT COALESCE(SUM(total_words), 0), COALESCE(SUM(total_duration_ms), 0), COALESCE(SUM(session_count), 0), COALESCE(SUM(transcription_count), 0) FROM stats",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )?;

        let average_wpm = if total_duration_ms > 0 {
            (total_words as f64) / (total_duration_ms as f64 / 60000.0)
        } else {
            0.0
        };

        Ok(UsageStats {
            total_words,
            total_duration_ms,
            total_sessions,
            total_transcriptions,
            average_wpm,
        })
    }

    pub fn get_daily_stats(&self, days: u32) -> Result<Vec<DailyStats>> {
        let conn = self.connection();
        let mut stmt = conn.prepare(
            "SELECT date, total_words, total_duration_ms, session_count, transcription_count
             FROM stats ORDER BY date DESC LIMIT ?1",
        )?;

        let stats = stmt
            .query_map(rusqlite::params![days], |row| {
                Ok(DailyStats {
                    date: row.get(0)?,
                    total_words: row.get(1)?,
                    total_duration_ms: row.get(2)?,
                    session_count: row.get(3)?,
                    transcription_count: row.get(4)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(stats)
    }
}
