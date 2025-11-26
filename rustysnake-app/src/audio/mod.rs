use rodio::{Sink, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct AudioManager {
    _stream: OutputStream,
    sink: Sink,
}

impl AudioManager {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        
        Self { _stream, sink }
    }

    pub fn play_sound(&self, sound_file: &str) {
        if let Ok(file) = File::open(Path::new(sound_file)) {
            if let Ok(source) = Decoder::new(BufReader::new(file)) {
                self.sink.append(source);
            }
        }
    }

    pub fn play_background(&self, music_file: &str) {
        if let Ok(file) = File::open(Path::new(music_file)) {
            if let Ok(source) = Decoder::new_looped(BufReader::new(file)) {
                self.sink.append(source);
            }
        }
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume.max(0.0).min(1.0));
    }

    pub fn stop_all(&self) {
        self.sink.stop();
    }
}
