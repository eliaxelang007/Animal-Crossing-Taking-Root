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

    pub fn epoch_to_hour_start_ms(&self) -> f64 {
        let current_time = Date::new_0();
        
        Date::new_with_year_month_day_hr_min_sec_milli(
            current_time.get_full_year(), 
            current_time.get_month() as i32,
            current_time.get_date() as i32,
            current_time.get_hours() as i32,
            0,
            0,
            0
        ).get_time()
    }

    pub fn since_epoch_ms(&self) -> f64 {
        self.date_creation_time_ms + (
            self.clock.now() - self.performance_creation_time_ms
        )
    }
}