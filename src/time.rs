use web_sys::{Performance, js_sys::Date};

pub struct Clock {
    clock: Performance,
    date_creation_time_ms: f64,
    performance_creation_time_ms: f64 
}

impl Clock {
    pub fn new(clock: Performance) -> Self {
        Clock {
            performance_creation_time_ms: clock.now(),
            date_creation_time_ms: Date::new_0().get_time(),
            clock,
        }
    }

    pub fn since_epoch_ms(&self) -> f64 {
        self.date_creation_time_ms + (
            self.clock.now() - self.performance_creation_time_ms
        )
    }
}