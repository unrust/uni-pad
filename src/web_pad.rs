use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Gamepad, GamepadButton, GamepadEvent};

pub const MAX_DEVICES: usize = 8;
pub const MAX_DIGITAL: usize = 16;
pub const MAX_ANALOG: usize = 8;

pub struct GamepadContext {
    pads: Rc<RefCell<HashMap<u32, Gamepad>>>,
}

impl GamepadContext {
    pub fn new() -> Option<Self> {
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        let pads = Rc::new(RefCell::new(HashMap::new()));
        match navigator.get_gamepads() {
            Err(e) => {
                // the browser does not support gamepad API
                eprintln!(
                    "error - could not initialize gamepad API : {}",
                    e.as_string().unwrap()
                );
                return None;
            }
            Ok(gamepads) => {
                // initialize connected gamepad list
                for i in 0..gamepads.length() {
                    let value = gamepads.at(i as i32);
                    if !value.is_null() {
                        let pad = value.dyn_into::<Gamepad>().unwrap();
                        pads.borrow_mut().insert(pad.index(), pad);
                    }
                }
            }
        }
        let connected_pads = pads.clone();
        let gamepad_connected_listener =
            Closure::<dyn FnMut(_)>::new(move |event: GamepadEvent| {
                // detect when a gamepad is connected
                let gamepad = event.gamepad().unwrap();
                connected_pads.borrow_mut().insert(gamepad.index(), gamepad);
            });
        window
            .add_event_listener_with_callback(
                "gamepadconnected",
                gamepad_connected_listener.as_ref().unchecked_ref(),
            )
            .unwrap();
        gamepad_connected_listener.forget();
        let disconnected_pads = pads.clone();
        let gamepad_disconnected_listener =
            Closure::<dyn FnMut(_)>::new(move |event: GamepadEvent| {
                // detect when a gamepad is disconnected
                let gamepad = event.gamepad().unwrap();
                disconnected_pads.borrow_mut().remove(&gamepad.index());
            });
        window
            .add_event_listener_with_callback(
                "gamepaddisconnected",
                gamepad_disconnected_listener.as_ref().unchecked_ref(),
            )
            .unwrap();
        gamepad_disconnected_listener.forget();
        Some(Self { pads })
    }
    pub fn update(&mut self) {}
    pub fn is_connected(&mut self, player_num: i32) -> bool {
        self.pads.borrow().get(&(player_num as u32)).is_some()
    }
    pub fn axis(&mut self, player_num: i32, axis_num: i32) -> f32 {
        let coef = if axis_num == 1 { -1.0 } else { 1.0 };
        if let Some(pad) = self.pads.borrow().get(&(player_num as u32)) {
            let axis_value = pad.axes().at(axis_num).as_f64().unwrap();
            coef * axis_value as f32
        } else {
            0.0
        }
    }
    pub fn button(&mut self, player_num: i32, button_num: i32) -> bool {
        if let Some(pad) = self.pads.borrow().get(&(player_num as u32)) {
            let button = pad
                .buttons()
                .at(button_num)
                .dyn_into::<GamepadButton>()
                .unwrap();
            button.pressed()
        } else {
            false
        }
    }
}
