use bevy::prelude::*;

mod world;
use world::*;
mod player;
use player::*;

fn main() {
	App::build()
		// Setup some MSAA for later
		.insert_resource(Msaa { samples: 4 })
		// Setup up window size
		.insert_resource(WindowDescriptor {
			title: "Simple test scene".to_string(),
			width: 800.,
			height: 800.,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup.system())
		.add_startup_system(create_world.system())
		.add_startup_system(create_player.system())
		.run();
}

fn setup(
	mut commands: Commands,
) {
	// Lets setup a basic camera
	commands.spawn_bundle(PerspectiveCameraBundle {
		transform: Transform::from_matrix(Mat4::from_rotation_translation(
			Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
			Vec3::new(-7.0, 20.0, 4.0),
		)),
		..Default::default()
	});

	// And finally a light source
	commands.spawn_bundle(LightBundle {
		transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
		..Default::default()
	});
}
