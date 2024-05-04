use bevy::prelude::*;

fn main() {
	App::new().add_plugins(DefaultPlugins).add_systems(Startup, (spawn_floor, spawn_light, spawn_player,spawn_camera)).run();
}

fn spawn_player(mut commands: Commands, mut meshes:ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>)
{
	let player = PbrBundle {
		//mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),   !!! Deprecated !!!
		mesh: meshes.add(Cuboid::default()),
		material:materials.add(Color::BLUE),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..default()
	};

	commands.spawn(player);
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
fn spawn_camera(mut commands: Commands)
{
	let camera = Camera3dBundle {
		transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y), ..default()
	};
	commands.spawn(camera);
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