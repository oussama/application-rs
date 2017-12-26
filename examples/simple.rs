extern crate application;

use application::*;

fn main() {
    let config = AppConfig::new("Title Sample",(600,400));
    let mut app = App::new(config);
    app.run(||{

    });
}