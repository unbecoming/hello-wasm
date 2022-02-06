use wasm_bindgen::prelude::*;
use web_sys::Element;

#[macro_use]
mod macros;
mod subject;

#[wasm_bindgen(start)]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    use web_sys::Window;

    let window: Window = web_sys::window().expect("no global `window` exists");
    let user_agent: String = window
        .navigator()
        .user_agent()
        .expect("no user-agent found");

    log!("wasm_bindgen(start), user-agent is: {}", user_agent);
}

#[wasm_bindgen]
pub fn mangle_text(element: Element, base_text: String) {
    use subject::Subject;

    match element.text_content() {
        None => {}
        Some(curr_content) if curr_content.is_empty() => (),
        Some(curr_content) => {
            let subject: Subject = Subject {
                text: curr_content,
                base_text,
            };
            let updated: Subject = subject.mangle();
            element.set_text_content(Some(&updated.text))
        }
    }
}
