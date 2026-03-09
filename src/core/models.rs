use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Surah {
    pub id: u16,
    pub name_ar: String,
    pub name_en: String,
    pub total_ayahs: u16,
    pub status: SurahStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SurahStatus {
    NotDownloaded,
    Downloaded,
    Downloading,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ayah {
    pub surah_id: u16,
    pub ayah_number: u16,
    pub arabic: String,
    pub romanized: String,
    pub english: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: i64,
    pub surah_id: u16,
    pub ayah_number: u16,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    Paginated,
    Scroll,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Popup {
    None,
    SurahList,
    Help,
    BookmarkList,
}
