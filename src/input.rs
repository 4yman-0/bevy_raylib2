use crate::prelude::*;
use bevy_app::*;

// TODO: Check raylib key changes and send events to bevy_input

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, bevy_input_system);
    }
}

#[derive(Event)]
struct MyEvent;

fn bevy_input_system(rl: RaylibHandle, event_writer: EventWriter<MyEvent>) {
	not_implemented!()
}
