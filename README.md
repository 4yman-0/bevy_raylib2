bevy_raylib2
===========

This is a Raylib plugin for Bevy.

Usage
-----

```rust
use bevy::prelude::*;
use bevy_raylib2::prelude::*;

fn main() {
    App::new()
    	.add_plugins(MinimalPlugins)
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

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
