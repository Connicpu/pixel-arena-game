use crate::Data;

pub struct Time {
    pub delta: f32,
    pub last_frame: f64,
    pub this_frame: f64,
}

impl Time {
    pub fn new() -> Self {
        Time {
            delta: 0.0,
            last_frame: time::precise_time_s(),
            this_frame: time::precise_time_s(),
        }
    }
}

#[derive(Default, conniecs::System)]
#[process = "update_time"]
pub struct UpdateTime;

fn update_time(_: &mut UpdateTime, data: &mut Data) {
    let time: &mut Time = &mut data.services.time;
    time.last_frame = time.this_frame;
    time.this_frame = time::precise_time_s();
    time.delta = (time.this_frame - time.last_frame) as f32;
}
