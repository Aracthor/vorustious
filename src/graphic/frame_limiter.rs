use std::time::Duration;
use std::time::Instant;
use std::thread::sleep;

pub struct FrameLimiter {
    clock: std::time::Instant,
    max_frame_time: f32,
}

impl FrameLimiter {
    pub fn new(max_framerate: f32) -> Self {
        Self {
            clock: Instant::now(),
            max_frame_time: 1.0 / max_framerate,
        }
    }

    pub fn limit(&mut self) {
        let time_to_sleep = self.max_frame_time - self.clock.elapsed().as_secs_f32();
        if time_to_sleep > 0.0 {
            sleep(Duration::from_secs_f32(time_to_sleep));
        }
        self.clock = Instant::now();
    }
}
