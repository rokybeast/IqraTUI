use crate::api::salah::{PrayerTimes, SalahApi};
use anyhow::Result;

pub struct SalahService {
    api: SalahApi,
    cached: Option<PrayerTimes>,
}

impl SalahService {
    pub fn new() -> Self {
        Self {
            api: SalahApi::new(),
            cached: None,
        }
    }

    pub async fn fetch(&mut self, lat: f64, lng: f64, method: u8) -> Result<PrayerTimes> {
        let times = self.api.fetch_prayer_times(lat, lng, method).await?;
        self.cached = Some(times.clone());
        Ok(times)
    }

    pub fn cached(&self) -> Option<&PrayerTimes> {
        self.cached.as_ref()
    }
}
