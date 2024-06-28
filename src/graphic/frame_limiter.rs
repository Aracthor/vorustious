use std::time::Duration;
use std::time::Instant;
use std::thread::sleep;

#[derive(Clone, Copy)]
pub struct FrameTimeInfo {
    pub min: f32,
    pub max: f32,
    pub average: f32,
}

impl FrameTimeInfo {
    pub fn reset() -> Self {
        Self {
            min: 0.0,
            max: 0.0,
            average: 0.0,
        }
    }
}

pub struct FrameLimiter {
    clock: std::time::Instant,
    max_frame_time: f32,
    last_frame_time: FrameTimeInfo,
    current_frame_time: FrameTimeInfo,
    registered_frame_times: i32,
}

impl FrameLimiter {
    pub fn new(max_framerate: f32) -> Self {
        Self {
            clock: Instant::now(),
            max_frame_time: 1.0 / max_framerate,
            last_frame_time: FrameTimeInfo::reset(),
            current_frame_time: FrameTimeInfo::reset(),
            registered_frame_times: 0,
        }
    }

    pub fn limit(&mut self) {
        let elapsed_time = self.clock.elapsed().as_secs_f32();
        let time_to_sleep = self.max_frame_time - elapsed_time;
        if time_to_sleep > 0.0 {
            sleep(Duration::from_secs_f32(time_to_sleep));
        }

        self.clock = Instant::now();
        self.current_frame_time.max = self.current_frame_time.max.max(elapsed_time);
        self.current_frame_time.min = if self.current_frame_time.min == 0.0 { elapsed_time} else { self.current_frame_time.min.min(elapsed_time) };
        self.current_frame_time.average += elapsed_time;

        const FRAMES_BY_INFO: i32 = 60;
        self.registered_frame_times += 1;
        if self.registered_frame_times >= FRAMES_BY_INFO {
            self.current_frame_time.average /= FRAMES_BY_INFO as f32;
            self.last_frame_time = self.current_frame_time;
            self.current_frame_time = FrameTimeInfo::reset();
            self.registered_frame_times = 0;
        }
    }

    pub fn frame_time(&self) -> FrameTimeInfo { self.last_frame_time }
}
