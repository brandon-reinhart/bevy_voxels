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

mod camera;
mod cube;

static GEN_CHUNKS_STAGE: &str = "gen_chunks";
static MESH_CHUNKS_STAGE: &str = "mesh_chunks";

const CHUNK_DIM:usize = 32;
const CHUNK_SIZE:usize = CHUNK_DIM*CHUNK_DIM*CHUNK_DIM;

#[derive(Default, Copy, Clone)]
pub struct Block(bool);

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

impl Chunk {
    fn voxel_coord( &self,idx: usize ) -> Vec3 {
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

    fn voxel_index( &self, coord: Vec3 ) -> ChunkIndex {
        if coord.x < 0.0 || coord.y < 0.0 || coord.z < 0.0
            || coord.x as usize >= CHUNK_DIM || coord.y as usize >= CHUNK_DIM || coord.z as usize >= CHUNK_DIM {
            ChunkIndex::Invalid
        } else {
            ChunkIndex::Index( (coord.z as usize * CHUNK_DIM * CHUNK_DIM) + (coord.y as usize * CHUNK_DIM) + coord.x as usize )
        }
    }

    fn voxel_at(&self, coord: Vec3) -> bool {
        let index = self.voxel_index(coord);
        match index {
            ChunkIndex::Invalid => false,
            ChunkIndex::Index(idx) => self.blocks[idx].0,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ChunkIndex {
    Invalid,
    Index(usize),
}

#[derive(Component)]
pub struct ChunkMesh;

fn generate_chunk(
    mut commands: Commands
)
{
    let mut rng = thread_rng();

    let mut blocks = [Block(false); CHUNK_SIZE];
    for mut block in &mut blocks {
        block.0 = rng.gen::<f32>() > 0.5;
    }

    commands
        .spawn()
        .insert(Chunk{ blocks } )
    ;
}

fn generate_face(
    coord: Vec3,
    face: usize,
    chunk_verts: &mut Vec<[f32;3]>,
    chunk_indices: &mut Vec<u32>,
    chunk_normals: &mut Vec<[f32;3]>,
    chunk_uvs: &mut Vec<[f32;2]>
) {
    let face_verts = cube::FACES[face].vertices;
    let mut verts = Vec::new();
    for face_vert in face_verts.iter() {
        let vertex = coord + Vec3::from_slice( face_vert );
        verts.push( vertex.to_array() );
    }

    let mut indices: Vec<u32> = cube::FACES[face].indices.clone().to_vec();

    indices.iter_mut().for_each(|x| *x += (chunk_verts.len()) as u32);
    chunk_indices.append(&mut indices);

    let mut normals = vec![
        cube::FACES[face].dir;verts.len()
    ];
    chunk_normals.append( &mut normals );

    let mut uvs = vec![
        [0.,0.];verts.len()
    ];

    chunk_uvs.append( &mut uvs );

    chunk_verts.append( &mut verts );
}

// https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/
fn mesh_chunk_cull_interior(
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
            let coord = chunk.voxel_coord(i);

            for face in 0..cube::FACES.len() {
                let neighbor = chunk.voxel_at(coord + Vec3::from_slice( &cube::FACES[face].dir.clone() ));
                if !neighbor {
                    generate_face( coord, face, &mut chunk_verts, &mut chunk_indices, &mut chunk_normals, &mut chunk_uvs );
                }
            }
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, chunk_verts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, chunk_normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, chunk_uvs);
    mesh.set_indices(Some(Indices::U32(chunk_indices)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.2, 0.5, 0.2, 1.0),
                //alpha_mode: AlphaMode::Mask(0.5),
                ..default()
            }),
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
        .add_startup_system_to_stage( MESH_CHUNKS_STAGE, mesh_chunk_cull_interior)
        .run();
}
