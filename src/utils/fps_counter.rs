use std::time::SystemTime;

pub struct FpsCounter {
    last_update: SystemTime,
    frame_ctr: u32,
    pub fps: u32,
    update_interval: f64,
}

#[allow(dead_code)]
impl FpsCounter {
    pub fn new(update_interval: f64) -> FpsCounter {
        FpsCounter {
            last_update: SystemTime::now(),
            frame_ctr: 0,
            fps: 0,
            update_interval,
        }
    }

    pub fn default() -> FpsCounter {
        FpsCounter {
            last_update: SystemTime::now(),
            frame_ctr: 0,
            fps: 0,
            update_interval: 1.0,
        }
    }

    pub fn on_update(&mut self) {
        self.frame_ctr += 1;
        let duration = SystemTime::now()
            .duration_since(self.last_update)
            .unwrap()
            .as_secs_f64();
        if duration >= self.update_interval {
            self.fps = (self.frame_ctr as f64 / duration) as u32;
            self.frame_ctr = 0;
            self.last_update = SystemTime::now();
        }
    }
}
