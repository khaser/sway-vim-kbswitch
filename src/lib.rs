extern crate swayipc;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use swayipc::{Connection, Input};

#[derive(Debug)]
enum Error {
    InconsistentLayouts,
    NoKeyboards,
}

#[no_mangle]
pub extern "C" fn Xkb_Switch_getXkbLayout() -> *const c_char {
    match Connection::new() {
        Ok(mut conn) => match get_cur_layout(&mut conn) {
            Ok(layout) => CString::new(layout).unwrap().into_raw() as *const c_char,
            Err(_) => 0 as *const c_char,
        },
        Err(_) => 0 as *const c_char,
    }
}

fn get_cur_layout(conn: &mut Connection) -> Result<String, Error> {
    let mut layouts: Vec<String> = get_keyboards(conn)
        .drain(..)
        .filter_map(|kb| kb.xkb_active_layout_name)
        .collect();
    layouts.dedup();
    match layouts.leak() {
        [] => Err(Error::NoKeyboards),
        [layout] => Ok(layout.to_string()),
        _ => Err(Error::InconsistentLayouts),
    }
}

fn get_keyboards(conn: &mut Connection) -> Vec<Input> {
    let mut all_inputs = conn.get_inputs().unwrap_or_default();
    all_inputs.retain(|input_device| input_device.input_type == "keyboard");
    all_inputs
}

#[no_mangle]
pub extern "C" fn Xkb_Switch_setXkbLayout(layout_ptr: *const c_char) {
    match Connection::new() {
        Ok(mut conn) => {
            let layout = unsafe { CStr::from_ptr(layout_ptr).to_string_lossy().to_string() };
            switch_layout(&mut conn, &layout);
        }
        Err(_) => (),
    };
}

fn switch_layout(conn: &mut Connection, layout: &String) {
    get_keyboards(conn).iter().for_each(|kb| {
        let layout_index = kb
            .xkb_layout_names
            .iter()
            .position(|x| x == layout)
            .unwrap();

        conn.run_command(format!(
            "input {} xkb_switch_layout {}",
            kb.identifier, layout_index
        ));
        // .unwrap();
    });
}
