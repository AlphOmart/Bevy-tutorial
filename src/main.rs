use bevy::prelude::*;

mod player;
mod camera;
mod world;

use camera::CameraPlugin;
use player::PlayerPlugin;
use world::FloorPlugin;

fn main() {
	App::new()
		.add_plugins((DefaultPlugins, PlayerPlugin, CameraPlugin, FloorPlugin))
		.run();
}
