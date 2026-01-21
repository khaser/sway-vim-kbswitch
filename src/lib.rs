extern crate swayipc;

use std::ffi::{CStr, CString};

use std::os::raw::c_char;

use swayipc::{Connection, Input};

#[derive(Debug)]
enum Error {
    NoKeyboards,
}

#[no_mangle]
pub extern "C" fn Xkb_Switch_getXkbLayout() -> *const c_char {
    let mut conn = Connection::new().unwrap();
    let layout = get_cur_layout(&mut conn).unwrap();
    // let _ = UnixStream::from(conn).shutdown(Shutdown::Both);
    CString::new(layout).unwrap().into_raw()
}

fn get_cur_layout(conn: &mut Connection) -> Result<String, Error> {
    let keyboards = get_keyboards(conn);

    // Build counts as Vec<(layout_name, count)>
    let mut counts: Vec<(String, usize)> = Vec::new();
    for kb in &keyboards {
        if let Some(name) = kb.xkb_active_layout_name.clone() {
            if let Some((_, c)) = counts.iter_mut().find(|(n, _)| *n == name) {
                *c += 1;
            } else {
                counts.push((name, 1));
            }
        }
    }
    if counts.is_empty() {
        return Err(Error::NoKeyboards);
    }

    // Aggregate rank of a layout across keyboards: (sum_of_indices, present_count)
    let score = |layout: &str| -> (usize, usize) {
        let mut sum = 0usize;
        let mut present = 0usize;
        for kb in &keyboards {
            if let Some(i) = kb.xkb_layout_names.iter().position(|x| x == layout) {
                sum += i;
                present += 1;
            }
        }
        (sum, present)
    };

    // Choose best by: count desc, score(sum asc), present desc, name lex asc
    let mut best = counts[0].0.clone();
    let mut best_n = counts[0].1;
    let mut best_s = score(&best);

    for (name, n) in counts.into_iter().skip(1) {
        let s = score(&name);
        let better =
            n > best_n ||
            (n == best_n && (s.0 < best_s.0 ||
                             (s.0 == best_s.0 && (s.1 > best_s.1 ||
                                                  (s.1 == best_s.1 && name < best)))));
        if better {
            best = name;
            best_n = n;
            best_s = s;
        }
    }

    Ok(best)
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
            // let _ = UnixStream::from(conn).shutdown(Shutdown::Both);
        }
        Err(_) => (),
    };
}

fn switch_layout(conn: &mut Connection, layout: &String) {
    get_keyboards(conn).iter().for_each(|kb| {
        let Some(layout_index) = kb
            .xkb_layout_names
            .iter()
            .position(|x| x == layout)
        else { return; };

        let _ = conn.run_command(format!(
            "input {} xkb_switch_layout {}",
            kb.identifier, layout_index
        ));
        // .unwrap();
    });
}
