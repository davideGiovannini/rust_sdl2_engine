use sdl2::TimerSubsystem;

const INTERVAL: u32 = 1_000 / 60;

pub struct FpsCounter {
    before: u32,
    last_second: u32,
    fps: u16,
}

impl FpsCounter {
    pub fn new(ref mut timer: &mut TimerSubsystem) -> FpsCounter {
        FpsCounter {
            before: timer.ticks(),
            last_second: timer.ticks(),
            fps: 0u16,
        }
    }
    /// Returns a (bool, Option<u16>) which indicates respectively
    /// (shouldJumpAtBeginningOfLoop, Option<CurrentFPS>)
    ///
    pub fn tick(&mut self, ref mut timer: &mut TimerSubsystem) -> (bool, Option<u16>) {
        // Frame timing (bis)
        let now = timer.ticks();
        let dt = now - self.before; //expressed in milliseconds

        // If the time elapsed since the last frame is too small, wait out the
        // difference and try again.
        if dt < INTERVAL {
            timer.delay(INTERVAL - dt);
            return (true, None);
        }

        self.before = now;
        self.fps += 1;

        if now - self.last_second > 1_000 {
            let fps = self.fps;
            self.last_second = now;
            self.fps = 0;
            return (false, Some(fps));
        }
        return (false, None);
    }
}
