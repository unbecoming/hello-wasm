use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
pub fn alive() {
    update_tagline();
}

#[wasm_bindgen(start)]
pub fn run() -> () {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    use web_sys::Window;

    let window: Window = web_sys::window().expect("no global `window` exists");
    let user_agent: String = window
        .navigator()
        .user_agent()
        .expect("no user-agent found");

    log!("wasm_bindgen(start), user-agent is: {}", user_agent);

    update_tagline();
}

fn update_tagline() {
    use web_sys::{Document, Element, Window};
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let tagline: Option<Element> = document.get_element_by_id("tagline");

    match tagline {
        Some(t) => {
            let new_content: String = match t.text_content() {
                None => {
                    let up: UpdateChar = UpdateChar::rand_char(&1);
                    String::from(up.c)
                }
                Some(curr_content) if curr_content.len() < 2 => {
                    let up: UpdateChar = UpdateChar::rand_char(&1);
                    String::from(up.c)
                }
                Some(curr_content) => {
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

                    let updated = taint(curr_content)
                        .and_then(rcovr)
                        .and_then(rcovr)
                        .and_then(rcovr)
                        .and_then(rcovr)
                        .and_then(rcovr)
                        .and_then(rcovr)
                        .and_then(rcovr)
                        .unwrap_or(UpdateChar::fallback());

                    log!("Updating tagline to: [{}]", updated);
                    updated
                }
            };
            t.set_text_content(Some(&new_content))
        }
        None => log!("Unable to load tagline element."),
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
    fn fallback() -> String {
        String::from_utf8(UpdateChar::BASE.to_vec())
            .unwrap_or_else(|_| UpdateChar::FAILURE.to_string())
    }
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
}

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}
