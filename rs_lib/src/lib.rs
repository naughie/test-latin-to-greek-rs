mod unicode;

mod state;
use state::State;

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Case {
    Capital,
    Small,
}

#[derive(Debug, Clone, Copy)]
enum Breathing {
    None,
    Smooth,
    Rough,
}

#[derive(Debug, Clone, Copy)]
enum Accent {
    None,
    Grave,
    Acute,
    Circumflex,
}

#[derive(Debug, Clone, Copy)]
enum Subscript {
    None,
    Iota,
}

#[derive(Debug, Clone, Copy)]
enum Diaeresis {
    None,
    Some,
}

#[wasm_bindgen]
pub fn latin_to_greek(s: &str) -> String {
    let mut buffer = String::new();

    let mut state = State::initial();
    for &c in s.as_bytes() {
        state = state.rewrite_buffer(c, &mut buffer);
        if state.is_initial() {
            unsafe {
                buffer.as_mut_vec().push(c);
            }
        }
    }

    buffer
}
