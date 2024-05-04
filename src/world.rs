use bevy::prelude::*;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
	fn build(&self, app: &mut App)
	{
		app.add_systems(Startup, (spawn_floor, spawn_light));
	}
}

fn spawn_floor( mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>)
{
	let floor = PbrBundle
	{
		//mesh: meshes.add(Plane3d::from_size(15.0)),  !!! Deprecated !!!
		mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
		material: materials.add(Color::DARK_GREEN), ..default()
	};

	commands.spawn(floor);
}

fn spawn_light(mut commands: Commands)
{
	let light = PointLightBundle {
		point_light: PointLight {
			intensity: 200000.0,
			..default()
		},
		transform: Transform::from_xyz(0.0, 5.0, 0.0),
		..default()
	};
	commands.spawn(light);
}