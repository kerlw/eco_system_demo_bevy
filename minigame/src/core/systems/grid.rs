//! 网格系统实现
use super::hex_grid::*;
use bevy::color::palettes::css::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct SharedHexMesh(Handle<Mesh>);

/// 初始化网格和共享资源
pub fn setup_grid(
    mut commands: Commands,
    config: Res<HexGridConfig>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let size = config.size;
    commands.spawn((
        Name::new("HexGrid"),
        SpatialPartition::new(config.into_inner().clone()),
    ));

    commands.insert_resource(SharedHexMesh(create_hex_mesh(meshes, size)));
}

/// 渲染网格系统
pub fn render_grid_system(
    mut commands: Commands,
    config: Res<HexGridConfig>,
    shared_mesh: Res<SharedHexMesh>,
    mut materials: ResMut<Assets<HexagonBorderMaterial>>,
) {
    let mut grids = vec![];

    // 构建所有地图格数组
    for x in 0..config.width as i32 {
        for y in 0..config.height as i32 {
            let pos = Position { x, y };
            let center = grid_to_world(pos, &config);

            grids.push((
                Mesh2d(shared_mesh.0.clone()),
                Transform::from_translation(center),
                MeshMaterial2d(materials.add(HexagonBorderMaterial {
                    color: LIME.into(),
                    border_color: WHITE.into(),
                    border_width: 0.05,
                })),
                HexCell { hex: (x, y).into() },
            ));
        }
    }

    commands.spawn_batch(grids);
}
