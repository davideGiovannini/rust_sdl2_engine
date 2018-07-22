use std::time::{Duration, Instant};
use std::thread;

const INTERVAL: u64 = 1_000 / 60;

const PHYSIC_INTERVAL: u32 = 1_000 / 60;

pub struct FpsCounter {
    interval: Duration,
    before: Instant,
    last_second: Instant,
    init_time: Instant,
    fps: u16,
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            interval: Duration::from_millis(INTERVAL),
            before: Instant::now(),
            last_second: Instant::now(),
            init_time: Instant::now(),
            fps: 0u16,
        }
    }
    /// Returns a (bool, Option<u16>, u32) which indicates respectively
    /// (shouldJumpAtBeginningOfLoop, Option<CurrentFPS>, deltaTime in milliseconds)
    ///
    pub fn tick(&mut self) -> (bool, Option<u16>, u32) {
        // Frame timing (bis)
        let now = Instant::now();
        let dt = now - self.before;

        // If the time elapsed since the last frame is too small, wait out the
        // difference and try again.
        if dt < self.interval {
            thread::sleep(self.interval - dt);
            return (true, None, 0);
        }

        self.before = now;
        self.fps += 1;

        let elapsed = now - self.last_second;

        if elapsed > Duration::new(1, 0) {
            let fps = self.fps;
            self.last_second = now;
            self.fps = 0;

            return (false, Some(fps), PHYSIC_INTERVAL);
        }
        (false, None, PHYSIC_INTERVAL)
    }

    pub fn elapsed(&self) -> u64 {
        let elapsed = self.init_time.elapsed();
        elapsed.as_secs() * 1_000 + u64::from(elapsed.subsec_nanos()) / 1_000_000
    }
}
