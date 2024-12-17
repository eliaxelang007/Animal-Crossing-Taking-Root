use web_sys::{js_sys::Date, wasm_bindgen::JsValue, Performance};

pub trait DateExtension {
    fn copy_with(&self,
        year: Option<u32>,
        month: Option<i32>,
        day: Option<i32>,
        hr: Option<i32>,
        min: Option<i32>,
        sec: Option<i32>,
        milli: Option<i32>) -> Date;
}

impl DateExtension for Date {
    fn copy_with(
        &self,
        year: Option<u32>,
        month: Option<i32>,
        day: Option<i32>,
        hr: Option<i32>,
        min: Option<i32>,
        sec: Option<i32>,
        milli: Option<i32>
    ) -> Date {
        Date::new_with_year_month_day_hr_min_sec_milli(
            year.unwrap_or(self.get_full_year()), 
            month.unwrap_or(self.get_month() as i32), 
            day.unwrap_or(self.get_date() as i32), 
            hr.unwrap_or(self.get_hours() as i32), 
            min.unwrap_or(self.get_minutes() as i32), 
            sec.unwrap_or(self.get_seconds() as i32), 
            milli.unwrap_or(self.get_milliseconds() as i32)
        )
    }
}

#[derive(Clone)]
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

    fn epoch_to_hour_start_ms(&self) -> f64 {
        Date::new(
            &JsValue::from_f64(self.since_epoch_ms())
        ).copy_with(
            None, 
            None, 
            None, 
            None, 
            Some(0), 
            Some(0), 
            Some(0)
        ).get_time()
    }

    fn epoch_to_next_hour(&self) -> f64 {
        const HOUR_MILLISECONDS: f64 = 3600000.0;
        self.epoch_to_hour_start_ms() + HOUR_MILLISECONDS
    }

    fn since_epoch_ms(&self) -> f64 {
        let real_time_ms = self.date_creation_time_ms + (
            self.clock.now() - self.performance_creation_time_ms
        );

        fn min_to_millis(mins: f64) -> f64 {
            mins * 60000.0
        }

        // TODO: Remove this later on!
        let offset = min_to_millis(58.0) - min_to_millis(23.0);

        real_time_ms + offset
    }

    pub fn since_hour_start_ms(&self) -> f64 {
        self.since_epoch_ms() - self.epoch_to_hour_start_ms()
    }

    pub fn to_next_hour_ms(&self) -> f64 {
        self.epoch_to_next_hour() - self.since_epoch_ms()
    }
}