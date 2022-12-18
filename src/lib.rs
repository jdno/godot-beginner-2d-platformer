use gdnative::prelude::*;
use godot_logger::GodotLogger;
use log::{Level, LevelFilter};

/// Registers all exposed classes to Godot
fn init(_handle: InitHandle) {
    if let Err(error) = GodotLogger::builder()
        .default_log_level(Level::Debug)
        .add_filter("h2", LevelFilter::Error)
        .add_filter("hyper", LevelFilter::Error)
        .add_filter("tower", LevelFilter::Error)
        .init()
    {
        godot_warn!("{}", error.to_string());
    } else {
        log::debug!("Initialized godot-logger");
    }
}

// Macro that creates the entry-points of the dynamic library
godot_init!(init);
