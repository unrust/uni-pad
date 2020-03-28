use gamepad_rs::*;
pub use gamepad_rs::{
    ControllerInfo, ControllerState, ControllerStatus, MAX_ANALOG, MAX_DEVICES, MAX_DIGITAL,
};

pub struct GamepadContext {
    context: ControllerContext,
}

impl GamepadContext {
    pub fn new() -> Option<Self> {
        if let Some(mut context) = ControllerContext::new() {
            context.scan_controllers();
            Some(Self { context })
        } else {
            None
        }
    }
    pub fn is_connected(&mut self, player_num: i32) -> bool {
        self.context.state(player_num as usize).status == ControllerStatus::Connected
    }
    pub fn axis(&mut self, player_num: i32, axis_num: i32) -> f32 {
        self.context.state(player_num as usize).analog_state[axis_num as usize]
    }
    pub fn button(&mut self, player_num: i32, button_num: i32) -> bool {
        self.context.state(player_num as usize).digital_state[button_num as usize]
    }
    pub fn update(&mut self) {
        for i in 0..MAX_DEVICES {
            self.context.update(i);
        }
    }
}
