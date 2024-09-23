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

impl AudioManager {
    pub fn new_default_output(asset_manager: Rc<AssetManager>) -> Result<Self> {
        let (_stream, handle) = OutputStream::try_default()?;

        Ok(AudioManager {
            assets: asset_manager,
            _stream,
            handle,
        })
    }

    pub fn play_now(&self, audio_ref: AudioRef) {
        let source = self.assets.get_audio(audio_ref);
        self.handle
            .play_raw(source.clone().convert_samples())
            .unwrap();
    }
}
