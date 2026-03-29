extern crate bevy_app;
extern crate bevy_ecs;
extern crate bevy_raylib2;
extern crate bevy_utils;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_raylib2::prelude::*;

fn main() {
    App::new()
        .add_plugins(RaylibPlugin::default())
        .add_systems(PostUpdate, render)
        .run();
}

fn render(mut raylib: NonSendMut<RaylibHandle>, thread: NonSend<RaylibThread>) {
    let cursor = raylib.get_mouse_position();
    let mut d = raylib.begin_drawing(&thread);
    d.clear_background(Color::WHITE);
    d.draw_text(
        &format!("{}, {}", cursor.x, cursor.y),
        12,
        12,
        20,
        Color::BLACK,
    );
}
