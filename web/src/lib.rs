use wasm_bindgen::prelude::*;

use client_shared::App;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .run()
        .expect("Should run the App");
}
