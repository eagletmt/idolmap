extern crate env_logger;
extern crate idolmap;

fn main() {

    env_logger::init().expect("Failed to initialize env_logger");

    idolmap::aikatsu::update_all();
    idolmap::lovelive::update_all();
    idolmap::pripara::update_all();
}
