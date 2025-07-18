use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

/// 六边形网格单元组件
#[derive(Component)]
pub struct HexCell {
    pub hex: HexCoord,
}

// 自定义边框着色器
#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct HexagonBorderMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub border_color: LinearRgba,
    #[uniform(0)]
    pub border_width: f32,
}

impl Material2d for HexagonBorderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hexagon_border.wgsl".into()
    }

    // fn vertex_shader() -> ShaderRef {
    //     "shaders/hexagon_border.wgsl".into()
    // }
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
