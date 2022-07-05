pub const FRONT_VERTS:[[f32;3];4] = [
    // Front face
    [0.0, 0.0,  1.0],
    [1.0, 0.0,  1.0],
    [1.0,  1.0,  1.0],
    [0.0,  1.0,  1.0],
];

pub const BACK_VERTS:[[f32;3];4] = [
    // Back face
    [0.0, 0.0, 0.0],
    [0.0,  1.0, 0.0],
    [1.0,  1.0, 0.0],
    [1.0, 0.0, 0.0],
];

pub const TOP_VERTS:[[f32;3];4] = [
    // Top face
    [0.0,  1.0, 0.0],
    [0.0,  1.0,  1.0],
    [1.0,  1.0,  1.0],
    [1.0,  1.0, 0.0],
];

pub const BOTTOM_VERTS:[[f32;3];4] = [
    // Bottom face
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0,  1.0],
    [0.0, 0.0,  1.0],
];

pub const RIGHT_VERTS:[[f32;3];4] = [
    // Right face
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
    [1.0, 1.0, 1.0],
    [1.0, 0.0, 1.0],
];

pub const LEFT_VERTS:[[f32;3];4] = [
    // Left face
    [0.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 1.0],
    [0.0, 1.0, 0.0],
];

pub const INDICES:[u32;6]     = [0,   1,  2,  0,  2, 3];

pub const _FRONT_INDICES:[u32;6]     = [0,   1,  2,  0,  2, 3];
pub const _BACK_INDICES:[u32;6]      = [4,   5,  6,  4,  6, 7];
pub const _TOP_INDICES:[u32;6]       = [8,   9, 10,  8, 10, 11];
pub const _BOTTOM_INDICES:[u32;6]    = [12, 13, 14, 12, 14, 15,];
pub const _RIGHT_INDICES:[u32;6]     = [16, 17, 18, 16, 18, 19,];
pub const _LEFT_INDICES:[u32;6]      = [20, 21, 22, 20, 22, 23,];

pub const UP_DIR:[f32;3]            = [0.,  1.,  0.];
pub const DOWN_DIR:[f32;3]          = [0., -1.,  0.];
pub const LEFT_DIR:[f32;3]          = [-1.,  0.,  0.];
pub const RIGHT_DIR:[f32;3]         = [1., 0.,  0.];
pub const FRONT_DIR:[f32;3]         = [0.,  0.,  1.];
pub const BACK_DIR:[f32;3]          = [0.,  0., -1.];

/*
pub const TOP:usize = 0;
pub const BOTTOM:usize = 1;
pub const FRONT:usize = 2;
pub const BACK:usize = 3;
pub const LEFT:usize = 4;
pub const RIGHT:usize = 5;
*/

pub struct Face {
    pub vertices: [[f32;3];4],
    pub indices: [u32;6],
    pub dir: [f32;3],
    pub name: &'static str,
}

pub const FACES:[Face;6] = [
    Face { vertices: TOP_VERTS, indices: INDICES, dir: UP_DIR, name: "TOP" },
    Face { vertices: BOTTOM_VERTS, indices: INDICES, dir: DOWN_DIR, name: "BOTTOM" },
    Face { vertices: FRONT_VERTS, indices: INDICES, dir: FRONT_DIR, name: "FRONT" },
    Face { vertices: BACK_VERTS, indices: INDICES, dir: BACK_DIR, name: "BACK" },
    Face { vertices: RIGHT_VERTS, indices: INDICES, dir: RIGHT_DIR, name: "RIGHT" },
    Face { vertices: LEFT_VERTS, indices: INDICES, dir: LEFT_DIR, name: "LEFT" },
];
