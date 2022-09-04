pub struct FPSManager {
    delta_time: u128,
    fps: f32,

    last_time: u128,
}

impl FPSManager {
    pub fn new() -> FPSManager {
        FPSManager {
            delta_time: 0u128,
            fps: 0f32,
            last_time: 0u128
        }
    }

    pub fn update(&mut self, new_time: u128)
    {
        let dt = new_time - self.last_time;
        self.delta_time = dt;
        if dt != 0u128
        {
            self.fps = 1f32 / (dt as f32) * 1000000f32;
            self.last_time = new_time;
        }

        println!("FPS: {}", self.fps);
    } 
}