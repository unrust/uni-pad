extern crate uni_pad;

use uni_pad::*;

// bootstrap code for wasm target
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    main();
    Ok(())
}

pub fn main() {
    let mut controller = GamepadContext::new().unwrap();

    //for _ in 0..10 {
    loop {
        for i in 0..MAX_DEVICES as i32 {
            if controller.is_connected(i) {
                // let nb_buttons;
                // let nb_axis;
                // {
                //     let info = controller.info(i);
                //     nb_buttons = info.digital_count;
                //     nb_axis = info.analog_count;
                //     println!(
                //         "[{}] {} {} buttons {} axis",
                //         i, info.name, info.digital_count, info.analog_count
                //     );
                // }
                let nb_buttons = 4;
                let nb_axis = 4;
                {
                    print!("\tbuttons :\n\t  A  B  X  Y  Up Do Le Ri St Bk Lt Rt LB RB\n\t");
                    for j in 0..nb_buttons {
                        print!("  {}", if controller.button(i, j) { 1 } else { 0 });
                    }
                    println!();
                    print!(
                        "\taxis :\n\t  ThumbLX  ThumbLY  LTrigger RTrigger ThumbRX  ThumbRY \n\t"
                    );
                    for j in 0..nb_axis {
                        print!("  {:1.4}", controller.axis(i, j));
                    }
                    println!();
                }
            }
        }
    }
}
