//! A Raylib plugin for Bevy.
//!
//! This module integrates Raylib into Bevy.

//#![add_test("../README.md")]

#![no_std]
extern crate alloc;

use alloc::string::String;
use bevy_app::prelude::*;
//use bevy_ecs::prelude::*;
use raylib::prelude::*;

/// The bevy_raylib2 prelude
pub mod prelude {
    #[allow(ambiguous_glob_reexports)]
    pub use crate::*;
    pub use raylib::prelude::*;
}

/// Configuration for the Raylib window and plugin behavior.
///
/// This plugin replaces the Bevy app's runner with a Raylib-powered loop.
/// All configuration is applied to a RaylibBuilder during startup.
pub struct RaylibPlugin {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub vsync: bool,
}

impl Default for RaylibPlugin {
    fn default() -> Self {
        Self {
            width: 640,
            height: 480,
            title: String::from("bevy_raylib2"),
            vsync: true,
        }
    }
}

impl Plugin for RaylibPlugin {
    /// Registers the Raylib runner to override Bevy’s default runner.
    fn build(&self, app: &mut App) {
        app.set_runner(runner);
    }
}

impl From<&RaylibPlugin> for RaylibBuilder {
    /// Converts RaylibPlugin configuration into a Raylib window builder.
    fn from(from: &RaylibPlugin) -> Self {
        let mut builder = init();
        builder
            .title(from.title.as_str())
            .height(from.height)
            .width(from.width);

        if from.vsync {
            builder.vsync();
        }
        builder
    }
}

/// Custom Bevy runner that sets up Raylib and performs the render loop.
fn runner(mut app: App) -> AppExit {
    let builder = *app
        .get_added_plugins::<RaylibPlugin>()
        .first()
        .expect("Cannot get Raylib plugin");

    let (rl, thread) = RaylibBuilder::from(builder).build();

    app.world_mut().insert_non_send_resource(rl);
    app.world_mut()
        .insert_non_send_resource(RaylibThreadHandle(thread));

    // Main loop
    while app
        .world()
        .get_non_send_resource::<RaylibHandle>()
        .is_some_and(|handle| !handle.window_should_close())
        || app.should_exit().is_some()
    {
        app.update();
    }

    AppExit::Success
}

/// A wrapper around RaylibThread to implement Deref.
///
/// Required because RaylibThread does not implement Clone or Send
/// and must remain in a NonSend resource.
pub struct RaylibThreadHandle(RaylibThread);

impl ::core::ops::Deref for RaylibThreadHandle {
    type Target = RaylibThread;

    fn deref(&self) -> &RaylibThread {
        &self.0
    }
}
