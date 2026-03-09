use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

const BASE_URL: &str = "https://api.aladhan.com/v1";

#[derive(Debug, Clone)]
pub struct SalahApi {
    client: Client,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PrayerTimes {
    pub fajr: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub isha: String,
}

#[derive(Debug, Deserialize)]
struct TimingsResponse {
    data: TimingsData,
}

#[derive(Debug, Deserialize)]
struct TimingsData {
    timings: TimingsRaw,
}

#[derive(Debug, Deserialize)]
struct TimingsRaw {
    #[serde(rename = "Fajr")]
    fajr: String,
    #[serde(rename = "Sunrise")]
    sunrise: String,
    #[serde(rename = "Dhuhr")]
    dhuhr: String,
    #[serde(rename = "Asr")]
    asr: String,
    #[serde(rename = "Maghrib")]
    maghrib: String,
    #[serde(rename = "Isha")]
    isha: String,
}

impl SalahApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_prayer_times(
        &self,
        latitude: f64,
        longitude: f64,
        method: u8,
    ) -> Result<PrayerTimes> {
        let url = format!(
            "{}/timings?latitude={}&longitude={}&method={}",
            BASE_URL, latitude, longitude, method
        );
        let resp: TimingsResponse = self.client.get(&url).send().await?.json().await?;
        Ok(PrayerTimes {
            fajr: resp.data.timings.fajr,
            sunrise: resp.data.timings.sunrise,
            dhuhr: resp.data.timings.dhuhr,
            asr: resp.data.timings.asr,
            maghrib: resp.data.timings.maghrib,
            isha: resp.data.timings.isha,
        })
    }
}
