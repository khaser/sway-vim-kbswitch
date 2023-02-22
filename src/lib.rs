extern crate swayipc;

use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use swayipc::{Connection, Input};

fn get_keyboard(conn: &mut Connection) -> Input {
    let all_inputs = conn.get_inputs().unwrap();
    let keyboard_iter = all_inputs.iter().position(|input_device| input_device.input_type == "keyboard").unwrap();
    return all_inputs[keyboard_iter].clone();
}

fn get_cur_layout(conn: &mut Connection) -> String {
    return get_keyboard(conn).xkb_active_layout_name.unwrap();
}

fn switch_layout(conn: &mut Connection, layout: &String) {
    let keyboard = get_keyboard(conn);
    match keyboard.xkb_layout_names.iter().position(|x| x == layout) {
        Some(layout_index) => conn.run_command(format!("input {} xkb_switch_layout {}", keyboard.identifier, layout_index)),
        None => panic!("There is no required layout for keyboard")
    };
}

#[no_mangle]
pub extern "C" fn Xkb_Switch_getXkbLayout() -> *const c_char {
    let mut conn = Connection::new().unwrap();
    return CString::new(get_cur_layout(&mut conn)).unwrap().into_raw() as *const c_char;
}

#[no_mangle]
pub extern "C" fn Xkb_Switch_setXkbLayout(layout_ptr: *const c_char) {
    let mut conn = Connection::new().unwrap();
    unsafe {
        let layout = CStr::from_ptr(layout_ptr).to_string_lossy().to_string();
        switch_layout(&mut conn, &layout);
    };
}

// #[non_exhaustive]
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Input {
//     /// The identifier for the input device.
//     pub identifier: String,
//     /// The human readable name for the device.
//     pub name: String,
//     /// The vendor code for the input device.
//     pub vendor: i32,
//     /// The product code for the input device.
//     pub product: i32,
//     /// The device type.  Currently this can be keyboard, pointer, touch,
//     /// tablet_tool, tablet_pad, or switch.
//     #[serde(rename = "type")]
//     pub input_type: String,
//     /// (Only keyboards) The name of the active keyboard layout in use.
//     pub xkb_active_layout_name: Option<String>,
//     /// (Only keyboards) A list a layout names configured for the keyboard.
//     #[serde(default)]
//     pub xkb_layout_names: Vec<String>,
//     /// (Only keyboards) The index of the active keyboard layout in use.
//     pub xkb_active_layout_index: Option<i32>,
//     /// (Only libinput devices) An object describing the current device
//     /// settings. See below for more information.
//     pub libinput: Option<Libinput>,
// }
