use anyhow::Result;

use crate::api::quran::QuranApi;
use crate::core::models::{Ayah, Bookmark, Surah, SurahStatus};
use crate::storage::db::Database;

pub struct QuranService {
    db: Database,
    api: QuranApi,
}

impl QuranService {
    pub fn new(db: Database, api: QuranApi) -> Self {
        Self { db, api }
    }

    pub fn get_surah_list(&self) -> Result<Vec<Surah>> {
        self.db.get_all_surahs()
    }

    pub fn is_surah_downloaded(&self, surah_id: u16) -> bool {
        self.db.surah_has_ayahs(surah_id).unwrap_or(false)
    }

    pub fn get_ayah(&self, surah_id: u16, ayah_number: u16) -> Result<Option<Ayah>> {
        self.db.get_ayah(surah_id, ayah_number)
    }

    pub fn get_all_ayahs(&self, surah_id: u16) -> Result<Vec<Ayah>> {
        self.db.get_ayahs_for_surah(surah_id)
    }

    pub async fn download_surah(&self, surah_id: u16) -> Result<()> {
        let (arabic_ayahs, english_ayahs, roman_ayahs) = tokio::try_join!(
            self.api.fetch_surah_arabic(surah_id),
            self.api.fetch_surah_english(surah_id),
            self.api.fetch_surah_transliteration(surah_id),
        )?;

        let mut ayahs = Vec::new();
        for i in 0..arabic_ayahs.len() {
            ayahs.push(Ayah {
                surah_id,
                ayah_number: (i + 1) as u16,
                arabic: arabic_ayahs.get(i).cloned().unwrap_or_default(),
                romanized: roman_ayahs.get(i).cloned().unwrap_or_default(),
                english: english_ayahs.get(i).cloned().unwrap_or_default(),
            });
        }

        self.db.insert_ayahs(&ayahs)?;
        Ok(())
    }

    pub fn delete_surah(&self, surah_id: u16) -> Result<()> {
        self.db.delete_ayahs_for_surah(surah_id)
    }

    pub fn toggle_bookmark(&self, surah_id: u16, ayah_number: u16) -> Result<()> {
        if self.db.bookmark_exists(surah_id, ayah_number)? {
            self.db.delete_bookmark(surah_id, ayah_number)
        } else {
            self.db.insert_bookmark(surah_id, ayah_number)
        }
    }

    pub fn is_bookmarked(&self, surah_id: u16, ayah_number: u16) -> bool {
        self.db.bookmark_exists(surah_id, ayah_number).unwrap_or(false)
    }

    pub fn get_bookmarks(&self) -> Result<Vec<Bookmark>> {
        self.db.get_all_bookmarks()
    }

    pub async fn fetch_and_cache_surah_list(&self) -> Result<()> {
        let surahs = self.api.fetch_surah_list().await?;
        self.db.insert_surahs(&surahs)?;
        Ok(())
    }

    pub fn get_surah(&self, surah_id: u16) -> Result<Option<Surah>> {
        let surahs = self.db.get_all_surahs()?;
        Ok(surahs.into_iter().find(|s| s.id == surah_id))
    }

    pub fn get_downloaded_count(&self) -> usize {
        self.get_surah_list()
            .unwrap_or_default()
            .iter()
            .filter(|s| s.status == SurahStatus::Downloaded)
            .count()
    }
}
