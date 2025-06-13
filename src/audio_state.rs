use rodio::{source::SineWave, OutputStream, Sink, Source};

pub struct AudioState {
    sink: Sink,
    _stream: OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    playing: bool,
}

impl AudioState {
    pub fn new() -> Self {
        //println!("in new audio state");
        let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to initialize audio");
        let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");
        let tone = SineWave::new(440.0).repeat_infinite();
        sink.append(tone);
        sink.set_volume(1.0);
        AudioState { sink, _stream, stream_handle, playing: false }
    }

    /// Enable or disable the beep based on sound timer value
    pub fn update(&mut self, sound_timer: u8) {
        if sound_timer > 0 && !self.playing {
            let tone = SineWave::new(440.0).repeat_infinite();
            self.sink = Sink::try_new(&self.stream_handle).expect("Failed to create new sink");
            self.sink.append(tone);
            self.sink.set_volume(1.0);
            self.playing = true;
        } else if sound_timer == 0 && self.playing {
            self.sink.stop();
            self.playing = false;
        }
    }
}