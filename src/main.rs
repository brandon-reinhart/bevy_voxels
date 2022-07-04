use bevy::{
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::WgpuSettings,
        mesh::{Indices, MeshVertexAttribute},
        },
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    render::mesh::PrimitiveTopology
};
use rand::prelude::*;

mod camera;

static GEN_CHUNKS_STAGE: &str = "gen_chunks";
static MESH_CHUNKS_STAGE: &str = "mesh_chunks";


const CHUNK_DIM:usize = 16;
const CHUNK_SIZE:usize = CHUNK_DIM*CHUNK_DIM*CHUNK_DIM;

//pub struct Block(u8);
#[derive(Copy, Clone)]
pub struct Block(bool);

impl Default for Block {
    fn default() -> Self {
        Block (false)
    }
}

#[derive(Component)]
pub struct Chunk {
    blocks: [Block; CHUNK_SIZE],
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            blocks: [Block(false); CHUNK_SIZE],
        }
    }
}

#[derive(Component)]
pub struct ChunkMesh;

fn generate_chunk(
    mut commands: Commands
)
{
    let mut rng = thread_rng();

    let mut blocks = [Block(false); CHUNK_SIZE];
    for i in 0..CHUNK_SIZE {
        blocks[i] = Block(rng.gen::<f32>() > 0.5);
    }

    commands
        .spawn()
        .insert(Chunk{ blocks } )
    ;
}

fn voxel_coord( idx: usize ) -> Vec3 {
    let mut i = idx;
    let x = i%CHUNK_DIM;
    i -= x;
    i /= CHUNK_DIM;
    let y = i%CHUNK_DIM;
    i -= y;
    i /= CHUNK_DIM;
    let z = i%CHUNK_DIM;

    Vec3::new(x as f32, y as f32, z as f32)
}

// https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/
fn mesh_chunk_stupid_method(
    mut wireframe_config: ResMut<WireframeConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&Chunk>,
)
{
    if query.is_empty() {
        return
    }

    let chunk = query.single();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut chunk_verts:Vec<[f32;3]> = Vec::new();
    let mut chunk_indices:Vec<u32> = Vec::new();
    let mut chunk_normals:Vec<[f32;3]> = Vec::new();
    let mut chunk_uvs:Vec<[f32;2]> = Vec::new();

    for i in 0..chunk.blocks.len() {
        if chunk.blocks[i].0 {
            let coord = voxel_coord(i);

            let mut verts = vec![
                (coord + Vec3::new(0., 0., 1. )).to_array(),
                (coord + Vec3::new(1., 0., 1. )).to_array(),
                (coord + Vec3::new(1., 1., 1. )).to_array(),
                (coord + Vec3::new(0., 1., 1. )).to_array(),
                (coord + Vec3::new(0., 0., 0. )).to_array(),
                (coord + Vec3::new(0., 1., 0. )).to_array(),
                (coord + Vec3::new(1., 1., 0. )).to_array(),
                (coord + Vec3::new(1., 0., 0. )).to_array(),
            ];

            let mut indices: Vec<u32> = vec![
                0, 1, 2, 0, 2, 3,
                4, 5, 6, 4, 6, 7,
                3, 2, 5, 6, 5, 2,
                5, 0, 3, 5, 4, 0,
                2, 7, 6, 2, 1, 7,
                1, 4, 7, 1, 0, 4,
            ];
            indices.iter_mut().for_each(|x| *x += (i * verts.len()) as u32);
            chunk_indices.append(&mut indices);

            let mut normals = vec![
                [0., 0., 0.];verts.len()
            ];

            chunk_normals.append( &mut normals );

            let mut uvs = vec![
                [0.,0.];verts.len()
            ];

            chunk_uvs.append( &mut uvs );

            chunk_verts.append( &mut verts );
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, chunk_verts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, chunk_normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, chunk_uvs);
    mesh.set_indices(Some(Indices::U32(chunk_indices)));

    wireframe_config.global = true;

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.2, 0.5, 0.2, 1.0),
                ..default()
            }),
            ..default()
        })
        .insert(ChunkMesh);
    }

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "bevy_voxels".to_string(),
            resizable: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgba(0.35, 0.35, 0.35, 1.0)))
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_stage(GEN_CHUNKS_STAGE,SystemStage::parallel())
        .add_startup_stage(MESH_CHUNKS_STAGE, SystemStage::parallel())
        .add_startup_system_to_stage( GEN_CHUNKS_STAGE, generate_chunk)
        .add_startup_system_to_stage( MESH_CHUNKS_STAGE, mesh_chunk_stupid_method)
        .run();
}
