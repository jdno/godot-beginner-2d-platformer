use gdnative::prelude::*;
use godot_logger::GodotLogger;
use log::{Level, LevelFilter};

use crate::actors::{Actor, Player};

mod actors;

/// Registers all exposed classes to Godot
fn init(handle: InitHandle) {
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

    handle.add_class::<Actor>();
    handle.add_class::<Player>();
}

// Macro that creates the entry-points of the dynamic library
godot_init!(init);
