#![allow(non_snake_case)]

use dioxus_daisyui_admin_protal::app::App;
use log::{info, LevelFilter};

fn main() {

    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    // launch the web app
    dioxus_web::launch(App);
    info!("starting...")
}
