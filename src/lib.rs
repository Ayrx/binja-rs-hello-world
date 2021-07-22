use binaryninja::binaryview::BinaryView;
use binaryninja::command::register;
use binaryninja::logger;
use log::{info, LevelFilter};

fn hello_world(view: &BinaryView) {
    info!("hello world from Rust!");
}

#[no_mangle]
pub extern "C" fn UIPluginInit() -> bool {
    logger::init(LevelFilter::Info).unwrap();

    register(
        "Hello World!",
        "This is a \"Hello World\" Rust plugin",
        hello_world,
    );
    true
}

