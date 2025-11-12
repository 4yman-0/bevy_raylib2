//! A Raylib plugin for bevy.
#![no_std]
#![expect(missing_docs, reason = "Docmentation isn't written (yet)")]
extern crate alloc;
extern crate bevy_app;
extern crate bevy_ecs;
extern crate raylib;

use alloc::string::String;

use bevy_app::prelude::*;
//use bevy_ecs::prelude::*;
use raylib::prelude::*;

#[cfg(feature = "input")]
mod input;

pub mod prelude {
    pub use crate::{RaylibPlugin, RaylibThreadHandle};
    pub use raylib::prelude::*;

    #[cfg(feature = "input")]
    pub use input::*;
}

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
    fn build(&self, app: &mut App) {
        app.set_runner(runner);
    }
}

impl From<&RaylibPlugin> for RaylibBuilder {
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

fn runner(mut app: App) -> AppExit {
    let builder = *app.get_added_plugins::<RaylibPlugin>().first().unwrap();
    let (rl, thread) = RaylibBuilder::from(builder).build();
    app.world_mut().insert_non_send_resource(rl);
    app.world_mut()
        .insert_non_send_resource(RaylibThreadHandle(thread));

    let should_close = |app: &App| {
        app.world()
            .get_non_send_resource::<RaylibHandle>()
            .is_some_and(|handle| handle.window_should_close())
    };

    while !should_close(&app) {
        app.update();
    }

    AppExit::Success
}

pub struct RaylibThreadHandle(RaylibThread);

impl core::ops::Deref for RaylibThreadHandle {
    type Target = RaylibThread;

    fn deref(&self) -> &RaylibThread {
        &self.0
    }
}
