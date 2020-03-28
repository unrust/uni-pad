use stdweb::unstable::TryInto;

pub const MAX_DEVICES: usize = 8;
pub const MAX_DIGITAL: usize = 16;
pub const MAX_ANALOG: usize = 8;

pub struct GamepadContext {}

impl GamepadContext {
    pub fn new() -> Option<Self> {
        let result: bool = js! {
            window.pads=[];
            if (navigator.getGamepads === undefined) {
                console.log("warning : no gamepad support on this browser");
                return false;
            } else {
                window.addEventListener("gamepadconnected", function(e) {
                    if (e.gamepad) {
                        console.log("gamepad["+e.gamepad.index+"] id "+e.gamepad.id+" connected.");
                        window.pads[e.gamepad.index] = e.gamepad;
                    }
                });
                window.addEventListener("gamepaddisconnected", function(e) {
                    if (e.gamepad) {
                        console.log("gamepad["+e.gamepad.index+"] id "+e.gamepad.id+" disconnected.");
                        window.pads[e.gamepad.index] = undefined;
                    }
                });
                return true;
            }
        }.try_into()
            .unwrap();
        match result {
            false => None,
            true => Some(GamepadContext {}),
        }
    }
    pub fn update(&mut self) {}
    pub fn is_connected(&mut self, player_num: i32) -> bool {
        let connected: bool = js! {
            if (navigator.userAgent.toLowerCase().indexOf("chrome") != -1) {
                var gp = navigator.getGamepads();
                for (var i=0; i < gp.length; i++) {
                    if (gp[i]!=null) {
                        window.pads[gp[i].index]=gp[i];
                    }
                }
            }
            if ( @{player_num} < window.pads.length && window.pads[@{player_num}] ) {
                return true;
            } else {
                return false;
            }
        }.try_into()
            .unwrap();
        connected
    }

    pub fn axis(&mut self, player_num: i32, axis_num: i32) -> f32 {
        let x: f64 = js! {
            if (navigator.userAgent.toLowerCase().indexOf("chrome") != -1) {
                var gp = navigator.getGamepads();
                for (var i=0; i < gp.length; i++) {
                    if (gp[i]!=null) {
                        window.pads[gp[i].index]=gp[i];
                    }
                }
            }
            let coef = @{axis_num} == 1 ? -1 : 1;
            if ( @{player_num} < window.pads.length && window.pads[@{player_num}] ) {
                return coef*window.pads[@{player_num}].axes[@{axis_num}];
            } else {
                return 0.0;
            }
        }.try_into()
            .unwrap();
        x as f32
    }
    pub fn button(&mut self, player_num: i32, button_num: i32) -> bool {
        let ret = js! {
            if (navigator.userAgent.toLowerCase().indexOf("chrome") != -1) {
                var gp = navigator.getGamepads();
                for (var i=0; i < gp.length; i++) {
                    if (gp[i]!=null) {
                        window.pads[gp[i].index]=gp[i];
                    }
                }
            }
            if ( @{player_num} < window.pads.length && window.pads[@{player_num}] ) {
                var button = window.pads[@{player_num}].buttons[@{button_num}];
                if (typeof button == "object") {
                    return button.pressed;
                } else {
                    return button == 1.0;
                }
            } else {
                return false;
            }
        }.try_into()
            .unwrap();
        ret
    }
}
