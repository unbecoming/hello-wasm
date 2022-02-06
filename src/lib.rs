use wasm_bindgen::prelude::*;
use web_sys::Element;

#[macro_use]
mod macros;

#[wasm_bindgen(start)]
pub fn run() -> () {
    extern crate console_error_panic_hook;

    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    use web_sys::Window;

    let window: Window = web_sys::window().expect("no global `window` exists");
    let user_agent: String = window
        .navigator()
        .user_agent()
        .expect("no user-agent found");

    log!("wasm_bindgen(start), user-agent is: {}", user_agent);
}

#[wasm_bindgen]
pub fn mangle_text(element: Element, base_text: String) -> () {
    match element.text_content() {
        None => (),
        Some(curr_content) if curr_content.len() == 0 => (),
        Some(curr_content) => {
            let updated = UpdateChar::rcovr(curr_content)
                .and_then(UpdateChar::rcovr)
                .and_then(UpdateChar::rcovr)
                .and_then(UpdateChar::rcovr)
                .and_then(UpdateChar::rcovr)
                .and_then(UpdateChar::rcovr)
                .and_then(UpdateChar::rcovr)
                .and_then(UpdateChar::taint);

            match updated {
                Some(new_value) => {
                    log!("[{}] mangled to [{}]", base_text, new_value);
                    element.set_text_content(Some(&new_value))
                }
                None => {
                    log!("Failed to mangle. Setting base: [{}]", base_text);
                    element.set_text_content(Some(&base_text))
                }
            }
        }
    }
}

#[derive(Debug)]
struct UpdateChar {
    c: char,
    idx: usize,
}

trait Consts {
    const BASE: &'static [u8];
    const CHARSET: &'static [u8];
    const FAILURE: &'static str;
}

impl Consts for UpdateChar {
    const BASE: &'static [u8] = b"the earth is pregnant with the dead";
    const CHARSET: &'static [u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~_ ";
    const FAILURE: &'static str = "failure";
}

impl UpdateChar {
    fn rcvr_char(available_pos: &usize) -> UpdateChar {
        use rand::Rng;
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let pool_idx: usize =
            std::cmp::min(rng.gen_range(0..UpdateChar::BASE.len()), *available_pos - 1);

        UpdateChar {
            c: char::from(UpdateChar::BASE[pool_idx]),
            idx: pool_idx,
        }
    }

    fn rand_char(available_pos: &usize) -> UpdateChar {
        use rand::Rng;
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let pool_idx: usize = rng.gen_range(0..UpdateChar::CHARSET.len());
        let dest_idx: usize = rng.gen_range(0..*available_pos);

        UpdateChar {
            c: char::from(UpdateChar::CHARSET[pool_idx]),
            idx: dest_idx,
        }
    }

    fn taint(taint_me: String) -> Option<String> {
        let up: UpdateChar = UpdateChar::rand_char(&taint_me.len());

        let a = &taint_me[0..up.idx];
        let b = &taint_me[up.idx + 1..taint_me.len()];
        let c = [a, &String::from(up.c), b].concat();
        Some(c)
    }
    fn rcovr(recover_me: String) -> Option<String> {
        let recover: UpdateChar = UpdateChar::rcvr_char(&recover_me.len());
        let x = &recover_me[0..recover.idx];
        let y = &recover_me[recover.idx + 1..recover_me.len()];
        let z = [x, &String::from(recover.c), y].concat();
        Some(z)
    }
}
