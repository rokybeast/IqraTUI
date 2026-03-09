use std::path::PathBuf;

use anyhow::Result;
use rusqlite::Connection;

use crate::core::models::{Ayah, Bookmark, Surah, SurahStatus};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &PathBuf) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_tables()?;
        Ok(db)
    }

    pub fn data_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("iqra")
    }

    pub fn default_path() -> PathBuf {
        Self::data_dir().join("iqra.db")
    }

    fn init_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS surahs (
                id INTEGER PRIMARY KEY,
                name_ar TEXT NOT NULL,
                name_en TEXT NOT NULL,
                total_ayahs INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS ayahs (
                surah_id INTEGER NOT NULL,
                ayah_number INTEGER NOT NULL,
                arabic TEXT NOT NULL,
                romanized TEXT NOT NULL,
                english TEXT NOT NULL,
                PRIMARY KEY (surah_id, ayah_number)
            );

            CREATE TABLE IF NOT EXISTS bookmarks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                surah_id INTEGER NOT NULL,
                ayah_number INTEGER NOT NULL,
                timestamp TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );"
        )?;
        Ok(())
    }

    pub fn insert_surahs(&self, surahs: &[Surah]) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        for surah in surahs {
            tx.execute(
                "INSERT OR REPLACE INTO surahs (id, name_ar, name_en, total_ayahs) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![surah.id, surah.name_ar, surah.name_en, surah.total_ayahs],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_all_surahs(&self) -> Result<Vec<Surah>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name_ar, name_en, total_ayahs FROM surahs ORDER BY id")?;
        let surahs = stmt
            .query_map([], |row| {
                let id: u16 = row.get(0)?;
                Ok((
                    id,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, u16>(3)?,
                ))
            })?
            .filter_map(|r| r.ok())
            .map(|(id, name_ar, name_en, total_ayahs)| {
                let has_ayahs = self.surah_has_ayahs(id).unwrap_or(false);
                Surah {
                    id,
                    name_ar,
                    name_en,
                    total_ayahs,
                    status: if has_ayahs {
                        SurahStatus::Downloaded
                    } else {
                        SurahStatus::NotDownloaded
                    },
                }
            })
            .collect();
        Ok(surahs)
    }

    pub fn surah_has_ayahs(&self, surah_id: u16) -> Result<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM ayahs WHERE surah_id = ?1",
            rusqlite::params![surah_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn insert_ayahs(&self, ayahs: &[Ayah]) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        for ayah in ayahs {
            tx.execute(
                "INSERT OR REPLACE INTO ayahs (surah_id, ayah_number, arabic, romanized, english) VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![ayah.surah_id, ayah.ayah_number, ayah.arabic, ayah.romanized, ayah.english],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_ayah(&self, surah_id: u16, ayah_number: u16) -> Result<Option<Ayah>> {
        let mut stmt = self.conn.prepare(
            "SELECT surah_id, ayah_number, arabic, romanized, english FROM ayahs WHERE surah_id = ?1 AND ayah_number = ?2"
        )?;
        let ayah = stmt
            .query_row(rusqlite::params![surah_id, ayah_number], |row| {
                Ok(Ayah {
                    surah_id: row.get(0)?,
                    ayah_number: row.get(1)?,
                    arabic: row.get(2)?,
                    romanized: row.get(3)?,
                    english: row.get(4)?,
                })
            })
            .ok();
        Ok(ayah)
    }

    pub fn get_ayahs_for_surah(&self, surah_id: u16) -> Result<Vec<Ayah>> {
        let mut stmt = self.conn.prepare(
            "SELECT surah_id, ayah_number, arabic, romanized, english FROM ayahs WHERE surah_id = ?1 ORDER BY ayah_number"
        )?;
        let ayahs = stmt
            .query_map(rusqlite::params![surah_id], |row| {
                Ok(Ayah {
                    surah_id: row.get(0)?,
                    ayah_number: row.get(1)?,
                    arabic: row.get(2)?,
                    romanized: row.get(3)?,
                    english: row.get(4)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(ayahs)
    }

    pub fn delete_ayahs_for_surah(&self, surah_id: u16) -> Result<()> {
        self.conn.execute(
            "DELETE FROM ayahs WHERE surah_id = ?1",
            rusqlite::params![surah_id],
        )?;
        Ok(())
    }

    pub fn insert_bookmark(&self, surah_id: u16, ayah_number: u16) -> Result<()> {
        let timestamp = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO bookmarks (surah_id, ayah_number, timestamp) VALUES (?1, ?2, ?3)",
            rusqlite::params![surah_id, ayah_number, timestamp],
        )?;
        Ok(())
    }

    pub fn delete_bookmark(&self, surah_id: u16, ayah_number: u16) -> Result<()> {
        self.conn.execute(
            "DELETE FROM bookmarks WHERE surah_id = ?1 AND ayah_number = ?2",
            rusqlite::params![surah_id, ayah_number],
        )?;
        Ok(())
    }

    pub fn bookmark_exists(&self, surah_id: u16, ayah_number: u16) -> Result<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM bookmarks WHERE surah_id = ?1 AND ayah_number = ?2",
            rusqlite::params![surah_id, ayah_number],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn get_all_bookmarks(&self) -> Result<Vec<Bookmark>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, surah_id, ayah_number, timestamp FROM bookmarks ORDER BY timestamp DESC"
        )?;
        let bookmarks = stmt
            .query_map([], |row| {
                Ok(Bookmark {
                    id: row.get(0)?,
                    surah_id: row.get(1)?,
                    ayah_number: row.get(2)?,
                    timestamp: row.get(3)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(bookmarks)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            rusqlite::params![key, value],
        )?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let result = self.conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            rusqlite::params![key],
            |row| row.get(0),
        );
        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
