use std::ops::Sub;

use web_sys::{js_sys::Date, wasm_bindgen::JsValue, Performance};
use strum::{VariantArray, FromRepr};

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

    pub fn since_epoch(&self) -> TimeSpan {
        TimeSpan {
            millis: self.date_creation_time_ms + (
                self.clock.now() - self.performance_creation_time_ms
            )
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeSpan {
    millis: f64
}

impl Sub for TimeSpan {
    type Output = TimeSpan;

    fn sub(mut self, other: Self) -> Self::Output {
        self.millis -= other.millis;
        self
    }
}

impl TimeSpan {
    pub const fn from_secs_f64(secs: f64) -> Self {
        TimeSpan {
            millis: secs * 1000.0
        }
    }

    pub const fn from_millis_f64(millis: f64) -> Self {
        TimeSpan {
            millis
        }
    }

    fn last_as_date(&self, unit: TimeUnit) -> Date {
        let mut as_date = self.as_date();

        if let Some(truncation_unit) =  
            TimeUnit::from_repr((unit as u8) + 1) {
            as_date.right_truncate(truncation_unit);
        }

        as_date
    }

    pub fn last(&self, unit: TimeUnit) -> TimeSpan {
        TimeSpan{
            millis: self.last_as_date(unit).get_time()
        }
    }

    pub fn next(&self, unit: TimeUnit) -> TimeSpan {
        let mut last = self.last_as_date(unit);
        let next_value = last.get(unit) + 1;

        TimeSpan {
            millis: last
                .set(next_value, unit)
                .get_time()
        }
    }

    pub fn since_last(&self, unit: TimeUnit) -> TimeSpan {
        *self - self.last(unit)
    }

    pub fn to_next(&self, unit: TimeUnit) -> TimeSpan {
        self.next(unit) - *self
    }

    pub fn as_secs_f64(&self) -> f64 {
        self.as_millis_f64() / 1000.0
    }

    pub fn as_millis_f64(&self) -> f64 {
        self.millis
    }

    pub fn as_date(&self) -> Date {
        Date::from_millis(self.millis)
    }
}

impl DateExtension for Date {
    fn from_millis(millis: f64) -> Self {
        Date::new(&JsValue::from_f64(millis))
    }

    fn set(&mut self, time: u32, unit: TimeUnit) -> &mut Self {
        use TimeUnit as T;

        match unit {
            T::Year => self.set_full_year(time),
            T::Month => self.set_month(time),
            T::Day => self.set_date(time),
            T::Hour => self.set_hours(time),
            T::Minute => self.set_minutes(time),
            T::Second => self.set_seconds(time),
            T::Millisecond => self.set_milliseconds(time)
        };

        self
    }

    fn get(&self, unit: TimeUnit) -> u32 {
        use TimeUnit as T;

        match unit {
            T::Year => self.get_full_year(),
            T::Month => self.get_month(),
            T::Day => self.get_date(),
            T::Hour => self.get_hours(),
            T::Minute => self.get_minutes(),
            T::Second => self.get_seconds(),
            T::Millisecond => self.get_milliseconds()
        }
    }

    fn truncate(&mut self, unit: TimeUnit) -> &mut Self {
        self.set(
            if let TimeUnit::Day = unit { 1 } else { 0 },
            unit
        )
    }

    fn right_truncate(&mut self, unit: TimeUnit) -> &mut Self {
        for &unit in &TimeUnit::VARIANTS[(unit as usize)..] {
            self.truncate(unit);
        }

        self
    }
}

#[derive(VariantArray, FromRepr, Clone, Copy)]
#[repr(u8)]
pub enum TimeUnit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond
} 

trait DateExtension {
    fn from_millis(millis: f64) -> Self;
    fn set(&mut self, time: u32, unit: TimeUnit) -> &mut Self;
    fn get(&self, unit: TimeUnit) -> u32;
    fn truncate(&mut self, unit: TimeUnit) -> &mut Self;
    fn right_truncate(&mut self, unit: TimeUnit) -> &mut Self;
}