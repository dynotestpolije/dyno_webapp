/// A TimeAxisScale represents a linear scale for timestamps within a fixed range.
/// A step duration is also expressed and indicates the interval to be used for each tick on the axis.
use dyno_core::chrono::{DateTime, Duration, Local, TimeZone, Utc};
use std::{ops::Range, rc::Rc};

use super::axis::{NormalisedValue, Scale, Tick};

/// An axis labeller is a closure that produces a string given a value within the axis scale
pub trait Labeller: Fn(i64) -> String {}

impl<T: Fn(i64) -> String> Labeller for T {}

fn local_time_labeller(format: &'static str) -> impl Labeller {
    move |ts| {
        let utc_date_time = Utc.timestamp_millis_opt(ts).unwrap();
        let local_date_time: DateTime<Local> = utc_date_time.into();
        local_date_time.format(format).to_string()
    }
}

#[derive(Clone)]
pub struct TimeScale {
    time: Range<i64>,
    step: i64,
    scale: f32,
    labeller: Option<Rc<dyn Labeller>>,
}

impl TimeScale {
    /// Create a new scale with a range and step representing labels as a day and month in local time.
    pub fn new(range: Range<DateTime<Utc>>, step: Duration) -> TimeScale {
        Self::with_local_time_labeller(range, step, "%d-%b")
    }

    /// Create a new scale with a range and step and local time labeller with a supplied format.
    pub fn with_local_time_labeller(
        range: Range<DateTime<Utc>>,
        step: Duration,
        format: &'static str,
    ) -> TimeScale {
        Self::with_labeller(range, step, Some(Rc::from(local_time_labeller(format))))
    }

    /// Create a new scale with a range and step and custom labeller.
    pub fn with_labeller(
        range: Range<DateTime<Utc>>,
        step: Duration,
        labeller: Option<Rc<dyn Labeller>>,
    ) -> TimeScale {
        let time_from = range.start.timestamp_millis();
        let time_to = range.end.timestamp_millis();
        let delta = time_to - time_from;
        let scale = if delta != 0 { 1.0 / delta as f32 } else { 1.0 };
        let step = step.num_milliseconds();

        TimeScale {
            time: time_from..time_to,
            step,
            scale,
            labeller,
        }
    }
}

impl Scale for TimeScale {
    type Scalar = i64;

    fn ticks(&self) -> Vec<Tick> {
        TimeScaleInclusiveIter {
            time_from: self.time.start,
            time_to: self.time.end,
            step: self.step,
            first_time: true,
        }
        .map(move |i| {
            let location = (i - self.time.start) as f32 * self.scale;
            Tick {
                location: NormalisedValue(location),
                label: self.labeller.as_ref().map(|l| (l)(i)),
            }
        })
        .collect()
    }

    fn normalise(&self, value: Self::Scalar) -> NormalisedValue {
        NormalisedValue((value - self.time.start) as f32 * self.scale)
    }
}

struct TimeScaleInclusiveIter {
    pub time_from: i64,
    pub time_to: i64,
    pub step: i64,
    pub first_time: bool,
}

impl Iterator for TimeScaleInclusiveIter {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let time = if !self.first_time {
            self.time_from.checked_add(self.step).map(|time| {
                self.time_from = time;
                time
            })
        } else {
            self.first_time = false;
            Some(self.time_from)
        };
        match self.step {
            s if s > 0 => time.filter(|t| *t <= self.time_to),
            s if s < 0 => time.filter(|t| *t >= self.time_to),
            _ => None,
        }
    }
}
