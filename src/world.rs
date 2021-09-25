use bevy::prelude::*;

pub fn create_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Lets create our WorldPlane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 8.0 })),
        material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
        transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
        ..Default::default()
    });


}
