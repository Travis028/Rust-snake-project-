use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub enum SoundEffect {
    Eat,
    GameOver,
}

pub struct AudioManager {
    _stream: OutputStream,
    sink: Sink,
    eat_sound: Arc<Vec<u8>>,
    game_over_sound: Arc<Vec<u8>>,
}

impl AudioManager {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // In a real app, you would load these from files.
        // For now, we'll use placeholders. You'll need to create these sound files.
        let eat_sound = Self::load_sound("assets/sounds/eat.ogg");
        let game_over_sound = Self::load_sound("assets/sounds/game_over.ogg");

        Self {
            _stream,
            sink,
            eat_sound,
            game_over_sound,
        }
    }

    pub fn play(&self, effect: SoundEffect) {
        let sound_data = match effect {
            SoundEffect::Eat => self.eat_sound.clone(),
            SoundEffect::GameOver => self.game_over_sound.clone(),
        };
        let source = Decoder::new(std::io::Cursor::new(sound_data)).unwrap();
        self.sink.append(source);
    }

    fn load_sound(path: &str) -> Arc<Vec<u8>> {
        // This is a placeholder. You'll need to create actual sound files.
        // If the file doesn't exist, it will create an empty sound.
        let file = File::open(path).ok();
        let data = file.map(|f| std::io::Read::bytes(BufReader::new(f)).map(|b| b.unwrap()).collect()).unwrap_or_default();
        Arc::new(data)
    }
}