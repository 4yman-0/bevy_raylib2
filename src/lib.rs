//! A Raylib plugin for bevy.
#![expect(missing_docs, reason = "Docmentation isn't written (yet)")]
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use raylib::prelude::*;

mod context;
pub use context::RaylibContext;

pub mod prelude {
    pub use crate::{Cursor, RaylibContext, RaylibPlugin, WindowConfig};
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
    let raylib_context = {
        let (rl, thread) = raylib::init()
            .size(window_config.width, window_config.height)
            .title(&window_config.title)
            .build();
        RaylibContext { rl, thread }
    };
    app.world_mut().insert_non_send_resource(raylib_context);

    let should_close = |app: &App| {
        app.world()
            .get_non_send_resource::<RaylibContext>()
            .is_some_and(|context| context.rl.window_should_close())
    };

    while !should_close(&app) {
        app.update();
    }

    AppExit::Success
}

pub fn update_cursor(raylib_context: NonSend<RaylibContext>, mut cursor: ResMut<Cursor>) {
    *cursor = {
        let Vector2 { x, y } = raylib_context.rl.get_mouse_position();
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
