/// A LinearScale represents a linear scale for floating point values within a fixed range.
/// A step is also expressed and indicates the interval to be used for each tick on the axis.
use std::{ops::Range, rc::Rc};

use super::axis::{NormalisedValue, Scale, Tick};

/// An axis labeller is a closure that produces a string given a value within the axis scale
pub trait Labeller: Fn(f32) -> String {}

impl<T: Fn(f32) -> String> Labeller for T {}

fn labeller() -> impl Labeller {
    |v| (v as i32).to_string()
}

#[derive(Clone)]
pub struct LinearScale {
    range: Range<f32>,
    step: f32,
    scale: f32,
    labeller: Option<Rc<dyn Labeller>>,
}

impl LinearScale {
    /// Create a new scale with a range and step and labels as a integers
    pub fn new(range: Range<f32>, step: f32) -> LinearScale {
        Self::with_labeller(range, step, Some(Rc::from(labeller())))
    }

    /// Create a new scale with a range and step and a custom labeller
    pub fn with_labeller(
        range: Range<f32>,
        step: f32,
        labeller: Option<Rc<dyn Labeller>>,
    ) -> LinearScale {
        let delta = range.end - range.start;
        let scale = if delta != 0.0 { 1.0 / delta } else { 1.0 };
        LinearScale {
            range,
            step,
            scale,
            labeller,
        }
    }
}

impl Scale for LinearScale {
    type Scalar = f32;

    fn ticks(&self) -> Vec<Tick> {
        LinearScaleInclusiveIter {
            from: self.range.start,
            to: self.range.end,
            step: self.step,
            first_time: true,
            last_time: false,
        }
        .map(move |v| {
            let location = (v - self.range.start) * self.scale;
            Tick {
                location: NormalisedValue(location),
                label: self.labeller.as_ref().map(|l| (l)(v)),
            }
        })
        .collect()
    }

    fn normalise(&self, value: Self::Scalar) -> NormalisedValue {
        NormalisedValue((value - self.range.start) * self.scale)
    }
}

struct LinearScaleInclusiveIter {
    pub from: f32,
    pub to: f32,
    pub step: f32,
    pub first_time: bool,
    pub last_time: bool,
}

impl Iterator for LinearScaleInclusiveIter {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.first_time {
            self.from += self.step;
        } else {
            self.first_time = false;
        };
        if (self.step >= 0.0 && self.from < self.to) || (self.step < 0.0 && self.from > self.to) {
            Some(self.from)
        } else if !self.last_time {
            self.last_time = true;
            Some(self.to)
        } else {
            None
        }
    }
}
