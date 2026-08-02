//! A Raylib plugin for Bevy.
#![no_std]
//#![add_test("../README.md")]

use bevy_app::prelude::*;
//use bevy_ecs::prelude::*;
use raylib::prelude::*;

/// The `bevy_raylib2` prelude
pub mod prelude {
    pub use crate::{RaylibPlugin};
}

/// Configuration for the Raylib window and plugin behavior.
///
/// This plugin replaces the Bevy app's runner with a Raylib-powered loop.
/// All configuration is applied to a `RaylibBuilder` during startup.
pub struct RaylibPlugin<'a>(pub RaylibBuilder<'a>);

impl Plugin for RaylibPlugin<'static> {
    /// Registers the Raylib runner to override Bevy’s default runner.
    fn build(&self, app: &mut App) {
        app.set_runner(runner);
    }
}

/// Custom Bevy runner that sets up Raylib and performs the render loop.
fn runner(mut app: App) -> AppExit {
    let builder = *app
        .get_added_plugins::<RaylibPlugin>()
        .first()
        .expect("Cannot get Raylib plugin");

    let (rl, thread) = builder.0.build();

    app.world_mut().insert_non_send_resource(rl);
    app.world_mut()
        .insert_non_send_resource(thread);

    // Main loop
    while app
        .world()
        .get_non_send_resource::<RaylibHandle>()
        .is_some_and(|handle| !handle.window_should_close())
        || app.should_exit().is_some()
    {
        app.update();
    }

    app.should_exit().unwrap_or(AppExit::Success)
}
