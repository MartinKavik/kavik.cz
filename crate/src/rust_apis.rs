use crate::app;
use seed;
use wasm_bindgen::prelude::*;
use std::convert::TryInto;

// ------ ------
//     Start
// ------ ------

// Called by /entries/index.ts to run the application.
// This function is called from /entries/index.ts
#[wasm_bindgen(start)]
pub fn run() {
    seed::App::build(app::init, app::update, app::view)
        .routes(|url| app::Msg::RouteChanged(url.try_into().ok()))
        .window_events(app::window_events)
        .finish()
        .run();
}
