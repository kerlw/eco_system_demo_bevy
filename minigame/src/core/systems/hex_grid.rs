use bevy::prelude::*;

/// 六边形网格单元组件
#[derive(Component)]
pub struct HexCell {
    pub hex: HexCoord,
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

/// 创建可复用的六边形网格
pub fn create_hex_mesh(mut meshes: ResMut<Assets<Mesh>>, size: f32) -> Handle<Mesh> {
    let mut mesh = Mesh::from(RegularPolygon::new(size, 6));
    mesh.rotate_by(Quat::from_rotation_z(std::f32::consts::FRAC_PI_6));
    meshes.add(mesh)
}

/// 六边形坐标系统
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

impl HexCoord {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
}

impl From<(i32, i32)> for HexCoord {
    fn from((q, r): (i32, i32)) -> Self {
        HexCoord { q, r }
    }
}