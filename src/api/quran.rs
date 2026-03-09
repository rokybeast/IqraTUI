use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

use crate::core::models::{Surah, SurahStatus};

const BASE_URL: &str = "https://api.alquran.cloud/v1";

#[derive(Debug, Clone)]
pub struct QuranApi {
    client: Client,
}

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
struct SurahListItem {
    number: u16,
    name: String,
    #[serde(rename = "englishName")]
    english_name: String,
    #[serde(rename = "numberOfAyahs")]
    number_of_ayahs: u16,
}

#[derive(Debug, Deserialize)]
struct SurahData {
    ayahs: Vec<AyahData>,
}

#[derive(Debug, Deserialize)]
struct AyahData {
    text: String,
}

impl QuranApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_surah_list(&self) -> Result<Vec<Surah>> {
        let url = format!("{}/surah", BASE_URL);
        let resp: ApiResponse<Vec<SurahListItem>> = self.client.get(&url).send().await?.json().await?;

        let surahs = resp
            .data
            .into_iter()
            .map(|s| Surah {
                id: s.number,
                name_ar: s.name,
                name_en: s.english_name,
                total_ayahs: s.number_of_ayahs,
                status: SurahStatus::NotDownloaded,
            })
            .collect();

        Ok(surahs)
    }

    pub async fn fetch_surah_arabic(&self, surah_id: u16) -> Result<Vec<String>> {
        let url = format!("{}/surah/{}/quran-uthmani", BASE_URL, surah_id);
        let resp: ApiResponse<SurahData> = self.client.get(&url).send().await?.json().await?;
        Ok(resp.data.ayahs.into_iter().map(|a| a.text).collect())
    }

    pub async fn fetch_surah_english(&self, surah_id: u16) -> Result<Vec<String>> {
        let url = format!("{}/surah/{}/en.asad", BASE_URL, surah_id);
        let resp: ApiResponse<SurahData> = self.client.get(&url).send().await?.json().await?;
        Ok(resp.data.ayahs.into_iter().map(|a| a.text).collect())
    }

    pub async fn fetch_surah_transliteration(&self, surah_id: u16) -> Result<Vec<String>> {
        let url = format!("{}/surah/{}/en.transliteration", BASE_URL, surah_id);
        let resp: ApiResponse<SurahData> = self.client.get(&url).send().await?.json().await?;
        Ok(resp.data.ayahs.into_iter().map(|a| a.text).collect())
    }
}
