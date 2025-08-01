//! 网格系统实现
use crate::scenes::GameSceneRoot;

use super::hex_grid::*;
use bevy::color::palettes::css::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MapGridRoot;

#[derive(Resource)]
pub struct SharedHexMesh(Handle<Mesh>);

/// 初始化网格和共享资源
pub fn setup_grid(
    mut commands: Commands,
    config: Res<HexGridConfig>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let size = config.size;
    commands.insert_resource(SpatialPartition::new(config.into_inner().clone()));
    commands.insert_resource(SharedHexMesh(create_hex_mesh(meshes, size)));
}

/// 渲染网格系统
pub fn render_grid_system(
    mut commands: Commands,
    mut partition: ResMut<SpatialPartition>,
    shared_mesh: Res<SharedHexMesh>,
    mut materials: ResMut<Assets<HexagonBorderMaterial>>,
    root: Query<Entity, With<GameSceneRoot>>,
) {
    info!("render hex grid {:?}", partition.config.clone());
    // let mut grids = vec![];

    let parent = commands
        .spawn((MapGridRoot, Transform::from_xyz(0.0, 0.0, 1.0)))
        .insert(ChildOf(root.single().unwrap()))
        .id();

    // 构建所有地图格数组
    for x in 0..partition.config.width as i32 {
        for y in 0..partition.config.height as i32 {
            let pos = Position { x, y };
            let center = grid_to_world(pos, partition.config.size);

            // grids.push((
            let cell = commands
                .spawn((
                    Mesh2d(shared_mesh.0.clone()),
                    Transform::from_translation(center),
                    MeshMaterial2d(materials.add(HexagonBorderMaterial {
                        color: LIME.into(),
                        border_color: WHITE.into(),
                        border_width: 0.05,
                    })),
                    HexCell { hex: (x, y).into() },
                ))
                .insert(ChildOf(parent))
                .id();
            // 将cell存入partition对应坐标下数组的第一个元素
            partition.insert(cell, pos);
        }
    }

    // commands.spawn_batch(grids);
}
