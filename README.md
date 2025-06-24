bevy_raylib2
===========

This is a Raylib plugin for Bevy.


Usage
-----

```rust
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_raylib2::prelude::*;
use bevy_utils::prelude::*;

fn main() {
    App::new()
        .add_plugins(RaylibPlugin)
        .insert_resource(WindowConfig {
            title: "Bevy + Raylib".to_owned(),
            ..default()
        })
        .add_systems(PostUpdate, render)
        .run();
}

fn render(
    mut raylib: NonSendMut<RaylibHandle>,
    thread: NonSend<RaylibThreadHandle>,
    cursor: Res<Cursor>,
) {
    let mut d = raylib.begin_drawing(thread.as_ref());
    d.clear_background(Color::WHITE);
    d.draw_text(
        &format!("{}, {}", cursor.x, cursor.y),
        12,
        12,
        20,
        Color::BLACK,
    );
}

```
