use std::{io::Cursor, rc::Rc};

use anyhow::Result;
use rodio::{source::Buffered, Decoder, OutputStream, OutputStreamHandle, Source};

use crate::assets::{manager::AssetManager, AudioRef};

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

    pub fn builder(&self, audio_ref: AudioRef) -> AudioBuilder {
        AudioBuilder {
            manager: self,
            audio: audio_ref,
            gain: 1.0,
            speed: 1.0,
        }
    }
}

impl<'a> AudioBuilder<'a> {
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