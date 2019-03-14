use winit::VirtualKeyCode;

pub struct KeyboardInput {
    pub rapid_tap_time: f32,
    keys: [KeyState; 256],
}

#[derive(Copy, Clone, Default)]
pub struct KeyState {
    pub pressed: bool,
    pub released: bool,
    pub held: bool,

    pub state_time: f32,
    pub consecutive_taps: u32,
    pub unfiltered_repeats: u32,
}

impl KeyboardInput {
    pub fn key(&self, vk: VirtualKeyCode) -> &KeyState {
        &self.keys[vk as usize]
    }

    pub fn tick(&mut self, dt: f32) {
        for key in self.keys.iter_mut() {
            key.state_time += dt;
            if key.state_time > self.rapid_tap_time {
                key.consecutive_taps = 0;
            }
        }
    }

    pub fn on_key_down(&mut self, vk: VirtualKeyCode) {
        let key = &mut self.keys[vk as usize];

        key.pressed = true;

        if !key.held && key.state_time <= self.rapid_tap_time {
            key.consecutive_taps += 1;
        }

        key.held = true;
        key.state_time = 0.0;
        key.unfiltered_repeats += 1;
    }

    pub fn on_key_up(&mut self, vk: VirtualKeyCode) {
        let key = &mut self.keys[vk as usize];

        key.released = true;
        key.held = false;
        key.state_time = 0.0;
        key.unfiltered_repeats = 0;
    }
}

impl Default for KeyboardInput {
    fn default() -> KeyboardInput {
        KeyboardInput {
            rapid_tap_time: 0.5,
            keys: [KeyState::default(); 256],
        }
    }
}

impl KeyState {
    pub fn pressed(&self, allow_repeats: bool) -> bool {
        self.pressed && (allow_repeats || self.unfiltered_repeats < 2)
    }

    pub fn released(&self) -> bool {
        self.released
    }

    pub fn held(&self) -> bool {
        self.held
    }

    pub fn held_for(&self, seconds: f32) -> bool {
        self.held && (self.state_time >= seconds)
    }
}
