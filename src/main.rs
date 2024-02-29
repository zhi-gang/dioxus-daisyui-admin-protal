#![allow(non_snake_case, unused)]

use dioxus_daisyui_admin_protal::app::App;
use log::{info, warn, LevelFilter};

fn main() {

    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    // launch the web app
    #[cfg(feature = "web")]
    dioxus_web::launch(App);

    #[cfg(feature = "desktop")]
    dioxus_desktop::launch(App);


    info!("starting...")
}
