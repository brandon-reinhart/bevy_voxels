use bevy::{
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::WgpuSettings,
        mesh::Indices,
        },
    pbr::wireframe::{Wireframe, WireframePlugin},
    render::mesh::PrimitiveTopology,
};
use rand::prelude::*;
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{greedy_quads, GreedyQuadsBuffer, MergeVoxel, Voxel, VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG};

mod camera;

static GEN_CHUNKS_STAGE: &str = "gen_chunks";
static MESH_CHUNKS_STAGE: &str = "mesh_chunks";

#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct BoolVoxel(bool);

const EMPTY: BoolVoxel = BoolVoxel(false);
const FULL: BoolVoxel = BoolVoxel(true);

impl Voxel for BoolVoxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if *self == EMPTY {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for BoolVoxel {
    type MergeValue = bool;

    fn merge_value(&self) -> Self::MergeValue {
        self.0
    }
}

const CHUNK_DIM:u32 = 18;
type ChunkShape = ConstShape3u32<18, 18, 18>;

#[derive(Component)]
pub struct Chunk {
    voxels: [BoolVoxel; ChunkShape::SIZE as usize],
}

#[derive(Component)]
pub struct ChunkMesh;

fn generate_chunk(
    mut commands: Commands
)
{
    let mut rng = thread_rng();

    let mut voxels = [EMPTY; ChunkShape::SIZE as usize];

    for x in 1..CHUNK_DIM-1 {
        for y in 1..CHUNK_DIM-1 {
            for z in 1..CHUNK_DIM-1 {
                let i = ChunkShape::linearize([x, y, z]);

                if rng.gen::<f32>() > 0.5 {
                    voxels[i as usize] = FULL;
                }
            }
        }
    }

    commands
        .spawn()
        .insert(Chunk{ voxels } )
    ;
}

fn mesh_chunk_greedy(
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
    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

    let mut buffer = GreedyQuadsBuffer::new(chunk.voxels.len());
    greedy_quads(
        &chunk.voxels,
        &ChunkShape {},
        [0; 3],
        [CHUNK_DIM-1; 3],
        &faces,
        &mut buffer
    );

    let num_indices = buffer.quads.num_quads() * 6;
    let num_vertices = buffer.quads.num_quads() * 4;

    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut tex_coords = Vec::with_capacity(num_vertices);

    for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
        for quad in group.into_iter() {
            indices.extend_from_slice(&face.quad_mesh_indices( positions.len() as u32 ));
            positions.extend_from_slice( &face.quad_mesh_positions( &quad, 1.0 ) );
            normals.extend_from_slice(&face.quad_mesh_normals());
            tex_coords.extend_from_slice(&face.tex_coords( RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, false, &quad ));
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);
    mesh.set_indices(Some(Indices::U32(indices)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.2, 0.5, 0.2, 1.0),
                //alpha_mode: AlphaMode::Mask(0.5),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::splat(-10.0)),
            ..default()
        })
        .insert(Wireframe)
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
        .add_startup_stage(GEN_CHUNKS_STAGE,SystemStage::parallel())
        .add_startup_stage(MESH_CHUNKS_STAGE, SystemStage::parallel())
        .add_startup_system_to_stage( GEN_CHUNKS_STAGE, generate_chunk)
        .add_startup_system_to_stage( MESH_CHUNKS_STAGE, mesh_chunk_greedy)
        .run();
}
