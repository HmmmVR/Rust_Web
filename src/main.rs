pub mod core;
pub mod app;

use core::router::Router;
use app::site::Site;

fn main() {
    let s = Site {
        msg: String::from("Hello world!")
    };

    s.log();
}

