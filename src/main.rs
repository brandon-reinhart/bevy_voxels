use bevy::{
    prelude::shape::Cube,
    prelude::*
};
use rand::prelude::*;

mod camera;

const SURFACE_WIDTH: i32 = 50;
const SURFACE_DEPTH: i32 = 50;
const CUBE_SIZE: f32 = 1.0;

const CUBE_DANCE_SPEED: f32 = 1.0;
const CUBE_DANCE_OFFSET: f32 = 0.25;

#[derive(Component)]
pub struct Block {
    dir: bool,
}

fn block_surface(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = thread_rng();

    let block = Mesh::from(Cube::new(CUBE_SIZE));
    let mut block_transform = Transform::from_xyz(
        -SURFACE_WIDTH as f32 / 2.0,
        0.0,
        -SURFACE_DEPTH as f32 / 2.0,
    );

    for _ in 0..SURFACE_WIDTH {
        for _ in 0..SURFACE_DEPTH {
            commands
                .spawn_bundle(PbrBundle {
                    transform: block_transform,
                    mesh: meshes.add(block.clone()),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(0.2, 0.5, 0.2, 1.0),
                        ..default()
                    }),
                    ..default()
                })
                .insert(Block {
                    dir: rng.gen::<f32>() < 0.5,
                });

            block_transform.translation.x += CUBE_SIZE;
            block_transform.translation.y = CUBE_DANCE_OFFSET * (rng.gen::<f32>() - 1.0);
        }
        block_transform.translation.x = -SURFACE_WIDTH as f32 / 2.0;
        block_transform.translation.z += CUBE_SIZE;
    }

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

fn block_dance(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Block)>
) {
    let time_delta = time.delta_seconds();

    for (mut transform, mut block) in query.iter_mut() {
        if block.dir {
            transform.translation.y -= CUBE_DANCE_SPEED * time_delta;
            if transform.translation.y < -CUBE_DANCE_OFFSET {
                block.dir = !block.dir;
            }
        } else {
            transform.translation.y += CUBE_DANCE_SPEED * time_delta;
            if transform.translation.y > CUBE_DANCE_OFFSET {
                block.dir = !block.dir;
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "bevy_voxels".to_string(),
            resizable: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgba(0.35, 0.35, 0.35, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_startup_system(block_surface)
        .add_system(block_dance)
        .run();
}
