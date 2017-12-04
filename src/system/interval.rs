use std::time::Duration;

use amethyst::core::timing::Stopwatch;
use specs::System;

/// Executes an inner system in a fixed interval.
pub struct IntervalSystem<S> {
    inner: S,
    stopwatch: Stopwatch,
    interval: Duration,
}

impl<S> IntervalSystem<S> {
    /// Wraps the given system to produce a new system that executes
    /// according to the given interval.
    pub fn wrap(system: S, interval: Duration) -> IntervalSystem<S> {
        IntervalSystem {
            inner: system,
            stopwatch: Stopwatch::new(),
            interval,
        }
    }
}

impl<'a, S> System<'a> for IntervalSystem<S> where S: System<'a> {
    type SystemData = <S as System<'a>>::SystemData;

    fn run(&mut self, data: Self::SystemData) {
        if self.stopwatch == Stopwatch::Waiting {
            self.stopwatch.start();
            return;
        }
        match self.stopwatch {
            ref mut sw @ Stopwatch::Started(_, _) => {
                if sw.elapsed() > self.interval {
                    self.inner.run(data);
                    sw.restart();
                }
            },
            ref mut sw @ _ => {
                sw.restart();
                return;
            }
        }
    }
}
