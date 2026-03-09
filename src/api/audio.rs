use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

const BASE_URL: &str = "https://api.quran.com/api/v4";

#[derive(Debug, Clone)]
pub struct AudioApi {
    client: Client,
}

#[derive(Debug, Deserialize)]
struct AudioResponse {
    audio_files: Vec<AudioFile>,
}

#[derive(Debug, Deserialize)]
struct AudioFile {
    #[serde(rename = "verse_key")]
    _verse_key: String,
    url: String,
}

impl AudioApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_ayah_audio_url(
        &self,
        reciter_id: u16,
        surah_id: u16,
        ayah_number: u16,
    ) -> Result<String> {
        let url = format!(
            "{}/recitations/{}/by_ayah/{}:{}",
            BASE_URL, reciter_id, surah_id, ayah_number
        );
        let resp: AudioResponse = self.client.get(&url).send().await?.json().await?;
        let audio = resp
            .audio_files
            .first()
            .ok_or_else(|| anyhow::anyhow!("No audio found"))?;

        let audio_url = if audio.url.starts_with("http") {
            audio.url.clone()
        } else {
            format!("https://audio.qurancdn.com/{}", audio.url)
        };

        Ok(audio_url)
    }

    pub async fn download_audio(&self, url: &str) -> Result<Vec<u8>> {
        let bytes = self.client.get(url).send().await?.bytes().await?;
        Ok(bytes.to_vec())
    }
}
