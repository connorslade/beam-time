use std::{io::Cursor, rc::Rc};

use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source, source::Buffered};

use crate::assets::{AudioRef, manager::AssetManager};

pub type AudioSource = Buffered<Decoder<Cursor<&'static [u8]>>>;

pub struct AudioManager {
    assets: Rc<AssetManager>,

    _stream: OutputStream,
    handle: OutputStreamHandle,
}

pub struct AudioBuilder<'a> {
    manager: &'a AudioManager,
    audio: AudioRef,

    gain: f32,
    speed: f32,
}

impl AudioManager {
    pub fn new_default_output(asset_manager: Rc<AssetManager>) -> Result<Self> {
        let (_stream, handle) = OutputStream::try_default()?;

        Ok(AudioManager {
            assets: asset_manager,
            _stream,
            handle,
        })
    }

    pub fn builder(&self, audio_ref: AudioRef) -> AudioBuilder<'_> {
        AudioBuilder {
            manager: self,
            audio: audio_ref,
            gain: 1.0,
            speed: 1.0,
        }
    }
}

impl AudioBuilder<'_> {
    pub fn with_gain(&mut self, gain: f32) -> &mut Self {
        self.gain = gain;
        self
    }

    pub fn with_speed(&mut self, speed: f32) -> &mut Self {
        self.speed = speed;
        self
    }

    pub fn play_now(&self) {
        let source = self.manager.assets.get_audio(self.audio);
        self.manager
            .handle
            .play_raw(
                source
                    .clone()
                    .convert_samples()
                    .amplify(self.gain)
                    .speed(self.speed),
            )
            .unwrap();
    }
}
