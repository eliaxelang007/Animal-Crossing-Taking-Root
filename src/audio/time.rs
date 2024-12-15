use web_sys::js_sys::Date;
use super::AudioContext;

pub struct Time {
    clock: AudioContext,
    date_creation_time_ms: f64,
    audio_creation_time_secs: f64 
}

impl Time {
    pub fn new(clock: AudioContext) -> Self {
        Time {
            audio_creation_time_secs: clock.0.current_time(),
            date_creation_time_ms: Date::new_0().get_time(),
            clock,
        }
    }

    pub fn since_epoch_ms(&self) -> f64 {
        self.date_creation_time_ms + (
            self.clock.0.current_time() - self.audio_creation_time_secs
        )
    }
}