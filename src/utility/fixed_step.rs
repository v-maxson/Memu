//! This is a modified version of the `fixedstep` crate. The original version can be found [here](https://github.com/HeroesGrave/fixedstep-rs/blob/master/src/lib.rs).
//! Note, this is a work in progress and is not used in this project yet.

use std::{time::{Duration, Instant}, cell::Cell};

const NANOS_PER_SECOND: f64 = 1_000_000_000.;

pub struct FixedStep {
    last_time: Cell<Instant>,
    update_interval: Cell<Duration>,
    accumulator: Cell<Duration>,
    update_counter: Cell<u32>,
    update_limit: Cell<u32>
}

impl FixedStep {
    /// Create and start a new FixedStep timer with the given frequency in Hz.
    pub fn start(hz: f64) -> Self {
        let seconds = 1.0 / hz;
        let full_seconds = seconds as u64;
        let remaining_nanos = (seconds.fract() * NANOS_PER_SECOND) as u32;

        FixedStep {
            last_time: Cell::new(Instant::now()),
            update_interval: Cell::new(Duration::new(full_seconds, remaining_nanos)),
            accumulator: Cell::new(Duration::new(0, 0)),
            update_counter: Cell::new(0),
            update_limit: Cell::new(3)
        }
    }

    /// Set the limit for how many updates can occur between rendering.
    /// This is the maximum number of times update() will return true between
    /// calls to render_delta.w
    /// 
    /// Use this if rendering on time is more important that keeping the simulation
    /// on time (which is usually the case for video games).
    pub fn limit(self, limit: u32) -> Self {
        self.update_limit.set(limit);
        self
    }

    /// Removes the update limit.
    pub fn unlimit(self) -> Self {
        self.update_limit.set(u32::MAX);
        self
    }

    /// Restarts the timer at the current time and clears any waiting updates.
    pub fn reset(&self) {
        self.last_time.set(Instant::now());
        self.update_counter.set(0);
        self.accumulator.set(Duration::new(0, 0));
    }

    pub fn update(&self) -> bool {
        let now = Instant::now();

        let accumulator = {
            let mut accumulator = self.accumulator.get();
            accumulator += now - self.last_time.get();
            self.accumulator.set(accumulator);
            accumulator
        };

        self.last_time.set(now);
        
        if accumulator >= self.update_interval.get() {
            // Time for another update
            let update_counter = {
                let mut update_counter = self.update_counter.get();
                update_counter += 1;
                self.update_counter.set(update_counter);
                update_counter
            };

            if update_counter > self.update_limit.get() {
                self.accumulator.set(Duration::new(0, 0));
                self.update_counter.set(0);

                false
            } else {
                self.accumulator.set(self.accumulator.get() - self.update_interval.get());
                true
            }
        } else {
            true
        }

    }

    pub fn render_delta(&self) -> f64 {
        fn duration_to_float(dur: Duration) -> f64 {
            (dur.as_secs() as f64 + dur.subsec_nanos() as f64 / NANOS_PER_SECOND)
        }

        self.update_counter.set(0);
        duration_to_float(self.accumulator.get()) / duration_to_float(self.update_interval.get())
    }

}