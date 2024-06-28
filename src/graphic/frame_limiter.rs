use std::time::Duration;
use std::time::Instant;
use std::thread::sleep;

pub struct FrameLimiter {
    clock: std::time::Instant,
    max_frame_time: f32,
    last_elapsed_time: f32,
}

impl FrameLimiter {
    pub fn new(max_framerate: f32) -> Self {
        Self {
            clock: Instant::now(),
            max_frame_time: 1.0 / max_framerate,
            last_elapsed_time: 0.0,
        }
    }

    pub fn limit(&mut self) {
        self.last_elapsed_time = self.clock.elapsed().as_secs_f32();
        let time_to_sleep = self.max_frame_time - self.last_elapsed_time;
        if time_to_sleep > 0.0 {
            sleep(Duration::from_secs_f32(time_to_sleep));
        }
        self.clock = Instant::now();
    }

    pub fn elapsed_time_secs(&self) -> f32 { self.last_elapsed_time }
}
