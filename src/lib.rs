//! A Raylib plugin for bevy.
#![expect(missing_docs, reason = "Docmentation isn't written (yet)")]
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use raylib::prelude::*;

pub mod prelude {
    pub use crate::{Cursor, RaylibThreadHandle, RaylibPlugin, WindowConfig};
    pub use raylib::prelude::*;
}

pub struct RaylibPlugin;

impl Plugin for RaylibPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_systems(PreUpdate, update_cursor)
            .set_runner(runner);
    }
}

fn runner(mut app: App) -> AppExit {
    let window_config = app
        .world_mut()
        .remove_resource::<WindowConfig>()
        .unwrap_or_default();
    let (rl, thread) = raylib::init()
        .size(window_config.width, window_config.height)
        .title(&window_config.title)
        .build();
    let world_mut = app.world_mut();
    world_mut.insert_non_send_resource(rl);
    world_mut.insert_non_send_resource(RaylibThreadHandle(thread));
    
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

impl AsRef<RaylibThread> for RaylibThreadHandle {
	fn as_ref(&self) -> &RaylibThread {
		&self.0
	}
}

pub fn update_cursor(raylib_handle: NonSend<RaylibHandle>, mut cursor: ResMut<Cursor>) {
    *cursor = {
        let Vector2 { x, y } = raylib_handle.get_mouse_position();
        Cursor { x, y }
    }
}

#[derive(Resource)]
pub struct WindowConfig {
    pub width: i32,
    pub height: i32,
    pub title: String,
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            width: 640,
            height: 480,
            title: "App".to_owned(),
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct Cursor {
    pub x: f32,
    pub y: f32,
}
