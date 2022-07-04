use bevy::prelude::*;
use bevy::prelude::shape::Cube;

mod camera;

fn single_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let block = Mesh::from( Cube::new( 2.0 ) );

    commands.spawn_bundle( PbrBundle {
        mesh: meshes.add( block ),
        material: materials.add(
            StandardMaterial {
                base_color: Color::rgba(0.2,0.5,0.2,1.0),
                ..default()
            } ),
        ..default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource( WindowDescriptor {
            title: "bevy_voxels".to_string(),
            resizable: true,
            ..default()
        })
        .insert_resource( ClearColor(Color::rgba(0.35, 0.35, 0.35, 1.0) ))
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_startup_system(single_block)
        .run();
}
