use std::io::Cursor;

use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use crate::api::audio::AudioApi;

#[derive(Debug, Clone, PartialEq)]
pub enum TtsState {
    Idle,
    Playing,
    Paused,
    Loading,
}

pub struct TtsPlayer {
    api: AudioApi,
    sink: Option<Sink>,
    _stream: Option<OutputStream>,
    _stream_handle: Option<OutputStreamHandle>,
    pub state: TtsState,
    pub reciter_id: u16,
    pub auto_next: bool,
    pub current_surah: u16,
    pub current_ayah: u16,
}

impl TtsPlayer {
    pub fn new(reciter_id: u16, auto_next: bool) -> Self {
        Self {
            api: AudioApi::new(),
            sink: None,
            _stream: None,
            _stream_handle: None,
            state: TtsState::Idle,
            reciter_id,
            auto_next,
            current_surah: 0,
            current_ayah: 0,
        }
    }

    fn ensure_output(&mut self) -> Result<()> {
        if self._stream.is_none() {
            let (stream, handle) = OutputStream::try_default()?;
            self._stream = Some(stream);
            self._stream_handle = Some(handle);
        }
        Ok(())
    }

    pub async fn play(&mut self, surah_id: u16, ayah_number: u16) -> Result<()> {
        self.stop();
        self.state = TtsState::Loading;
        self.current_surah = surah_id;
        self.current_ayah = ayah_number;

        let url = self
            .api
            .fetch_ayah_audio_url(self.reciter_id, surah_id, ayah_number)
            .await?;

        let audio_data = self.api.download_audio(&url).await?;

        self.ensure_output()?;

        if let Some(ref handle) = self._stream_handle {
            let sink = Sink::try_new(handle)?;
            let cursor = Cursor::new(audio_data);
            let source = Decoder::new(cursor)?;
            sink.append(source);
            self.sink = Some(sink);
            self.state = TtsState::Playing;
        }

        Ok(())
    }

    pub fn pause(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.pause();
            self.state = TtsState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.play();
            self.state = TtsState::Playing;
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
        self.state = TtsState::Idle;
    }

    pub fn toggle(&mut self) {
        match self.state {
            TtsState::Playing => self.pause(),
            TtsState::Paused => self.resume(),
            _ => {}
        }
    }

    pub fn is_finished(&self) -> bool {
        if let Some(ref sink) = self.sink {
            sink.empty()
        } else {
            true
        }
    }
}
