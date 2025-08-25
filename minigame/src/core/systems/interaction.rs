use std::f32::consts::PI;

use crate::core::entities::{OnMapEntitiesRoot, SharedBarMesh, spawn_entity};
use crate::core::hex_grid::SpatialPartition;
use crate::core::systems::hex_grid::{HexMapPosition, HexagonBorderMaterial};
use crate::level::config::EntityConfig;
use crate::scenes::LevelGold;
use crate::scenes::scene_selector::SceneSystemSet;
use crate::sprite::sprite_mgr;
use crate::ui::{CardSelectedMarker, EntityCardInfo, SelectedCardHolder, show_error_tips};
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct GlobalMousePosition {
    pub is_in_primary_window: bool,
    pub pos: Vec2,
}
#[derive(Resource, Default)]
pub struct SpecialMapCellHolder {
    pub selected: Option<Entity>,
    pub hovered: Option<Entity>,
}

// 动画曲线类型
#[derive(Resource, Clone, Copy)]
pub enum AnimationCurve {
    Linear,
    EaseInOut,
    Bounce,
}

impl Default for AnimationCurve {
    fn default() -> Self {
        Self::EaseInOut
    }
}

// 交互状态组件
#[derive(Component)]
pub struct MapCellSelectedMarker {
    pub timer: Timer,
}

#[derive(Component)]
pub struct MapCellHoveredMarker;

#[derive(Component)]
pub struct ClickEffect {
    pub timer: Timer,
    pub scale: f32,
}

// 交互配置资源
#[derive(Resource)]
pub struct MapCellEffectConfig {
    pub click_radius: f32,
    pub hover_radius: f32,
    pub click_effect_duration: f32,
    pub click_effect_scale: f32,
    pub particle_speed: f32,
    pub particle_count: u32,
    pub animation_curve: AnimationCurve,
}

impl Default for MapCellEffectConfig {
    fn default() -> Self {
        Self {
            click_radius: 43.3,
            hover_radius: 43.3,
            click_effect_duration: 0.5,
            click_effect_scale: 1.2,
            particle_speed: 200.0,
            particle_count: 8,
            animation_curve: AnimationCurve::default(),
        }
    }
}

// 颜色配置资源
#[derive(Resource)]
pub struct MapCellColors {
    pub normal: Color,
    pub hovered: Color,
    pub selected: Color,
    pub click_effect: Color,
}

impl Default for MapCellColors {
    fn default() -> Self {
        Self {
            normal: Color::srgb(0.1, 0.55, 0.2),
            hovered: Color::srgb(0.10, 0.80, 0.25),
            selected: Color::srgb(0.80, 0.45, 0.20),
            click_effect: Color::srgb(1.0, 1.0, 1.0),
        }
    }
}

pub fn update_global_mouse_position_system(
    mut mouse_position: ResMut<GlobalMousePosition>,
    windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(w) = windows.single() {
        if let Some(pos) = w.cursor_position() {
            mouse_position.is_in_primary_window = true;
            mouse_position.pos = pos;
            return;
        }
    }

    mouse_position.is_in_primary_window = false;
}

// 点击检测系统
// 1. 有选中卡片的情况下，点击到地图单元上尝试放置选中的卡片对应的实体
// 2. 无选中卡片的情况下，处理地图单元的选中/取消选中
pub fn map_cell_click_system(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_position: Res<GlobalMousePosition>,
    interaction: Res<MapCellEffectConfig>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    card_q: Query<&EntityCardInfo, With<CardSelectedMarker>>,
    cell_q: Query<(
        Entity,
        &HexMapPosition,
        &MeshMaterial2d<HexagonBorderMaterial>,
        Has<MapCellSelectedMarker>,
    )>,
    root_q: Query<Entity, With<OnMapEntitiesRoot>>,
    mut cell_holder: ResMut<SpecialMapCellHolder>,
    card_holder: Res<SelectedCardHolder>,
    mut partition: ResMut<SpatialPartition>,
    sprite_mgr: ResMut<sprite_mgr::SpriteManager>,
    mut materials: ResMut<Assets<HexagonBorderMaterial>>,
    colors: Res<MapCellColors>,
    mut level_gold: ResMut<LevelGold>,
    bar_mesh: Res<SharedBarMesh>,
) {
    if !mouse.just_pressed(MouseButton::Left) || !mouse_position.is_in_primary_window {
        return;
    }

    let cursor_pos = mouse_position.pos;
    // 利用camera的viewport_to_world_2d将鼠标位置坐标转换为视窗位置坐标
    if let Ok((camera, cam_transform)) = camera_q.single() {
        if let Ok(pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) {
            let selected_card = card_holder.0.clone();
            // 计算当前鼠标位置对应的地块坐标
            let cell_pos = partition.world_to_grid(&pos);
            // 地块坐标在地图范围内
            if partition.is_valid_position(&cell_pos) {
                if let Some(card) = selected_card {
                    if let Ok(card_info) = card_q.get(card) {
                        // 清除当前的选中地块
                        if let Some(selected_cell) = cell_holder.selected {
                            if let Ok((_, _, material, is_selected)) = cell_q.get(selected_cell) {
                                if is_selected {
                                    remove_cell_selected_mark(
                                        &mut commands,
                                        &mut cell_holder,
                                        &mut materials,
                                        &colors,
                                        material,
                                        selected_cell,
                                    );
                                }
                            }
                        }

                        if card_info.cost > level_gold.0 {
                            show_error_tips(&mut commands, "金币不足!");
                            return;
                        }

                        if !partition
                            .check_entity_conflict_by_pos(card_info.entity_type.clone(), &cell_pos)
                        {
                            show_error_tips(&mut commands, "位置冲突!");
                            return;
                        }

                        let parent = root_q.single().unwrap();
                        // 选择了卡片，则处理投放
                        // TODO 这里直接调用spawn会需要很多额外参数，可以考虑加入事件，避免这些参数冗余
                        spawn_entity(
                            &mut commands,
                            &EntityConfig {
                                entity_type: card_info.entity_type.clone(),
                                pos: cell_pos.to_vec2(),
                                health: None,
                                reproduction_rate: None,
                                growth_rate: None,
                                ..Default::default()
                            },
                            &sprite_mgr,
                            &bar_mesh,
                            &mut partition,
                            &parent,
                        );
                        level_gold.0 -= card_info.cost;
                    }
                } else {
                    if let Some(selected_cell) = cell_holder.selected {
                        if let Ok((_, old_pos, material, is_selected)) = cell_q.get(selected_cell) {
                            if old_pos.eq(&cell_pos) && is_selected {
                                // TODO 可以尝试reset effect的timer
                                return;
                            } else {
                                remove_cell_selected_mark(
                                    &mut commands,
                                    &mut cell_holder,
                                    &mut materials,
                                    &colors,
                                    material,
                                    selected_cell,
                                );
                            }
                        }
                    }
                    let entity = partition.get_cell_by_pos(&cell_pos);
                    if let Ok((entity, _, _, _)) = cell_q.get(entity) {
                        cell_holder.selected = Some(entity);
                        // 未选择卡片则处理选中地块
                        commands
                            .entity(entity)
                            .insert(MapCellSelectedMarker {
                                timer: Timer::from_seconds(2., TimerMode::Repeating),
                            })
                            .insert(ClickEffect {
                                timer: Timer::from_seconds(
                                    interaction.click_effect_duration,
                                    TimerMode::Once,
                                ),
                                scale: interaction.click_effect_scale,
                            });
                    }
                }
            } else if selected_card.is_none() {
                // 不在地图范围内时，仅处理未选中卡片的情况，取消已经设置了selected的地块组件
                if let Some(selected_cell) = cell_holder.selected {
                    if let Ok((_, _, material, is_selected)) = cell_q.get(selected_cell) {
                        if is_selected {
                            remove_cell_selected_mark(
                                &mut commands,
                                &mut cell_holder,
                                &mut materials,
                                &colors,
                                material,
                                selected_cell,
                            );
                        }
                    }
                }
            }
        }
    }
}

fn remove_cell_selected_mark(
    commands: &mut Commands,
    cell_holder: &mut SpecialMapCellHolder,
    materials: &mut Assets<HexagonBorderMaterial>,
    colors: &MapCellColors,
    material: &MeshMaterial2d<HexagonBorderMaterial>,
    selected_cell: Entity,
) {
    commands
        .entity(selected_cell)
        .remove::<MapCellSelectedMarker>()
        .remove::<ClickEffect>();
    cell_holder.selected = None;
    materials.get_mut(material.0.id()).map(|m| {
        m.color = colors.normal.to_linear();
        m.border_color = Color::srgb(1.0, 1.0, 1.0).to_linear();
        m.border_width = 0.05;
    });
}

// 悬停检测系统, 遍历所有的地图块效率太低。应当建立地图块的图元信息二维数组，进行精准定位计算
pub fn map_cell_hover_system(
    mut commands: Commands,
    mouse_position: Res<GlobalMousePosition>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(
        Entity,
        &MeshMaterial2d<HexagonBorderMaterial>,
        &HexMapPosition,
        Has<MapCellHoveredMarker>,
    )>,
    colors: Res<MapCellColors>,
    mut materials: ResMut<Assets<HexagonBorderMaterial>>,
    mut holder: ResMut<SpecialMapCellHolder>,
    partition: Res<SpatialPartition>,
) {
    if !mouse_position.is_in_primary_window {
        return;
    }

    let cursor_pos = mouse_position.pos;
    // 利用camera的viewport_to_world_2d将鼠标位置坐标转换为视窗位置坐标
    if let Ok((camera, cam_transform)) = camera_q.single() {
        if let Ok(pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) {
            // 获取与该位置最接近的HexMapPosition（可能是处于地图外的）
            let cell = partition.world_to_grid(&pos);

            // 已经存在hover的cell则判断是否和当前位置的cell相同，不同的时候则取消之前的hovered的cell
            if let Some(hover_cell) = holder.hovered {
                if let Ok((entity, material, pos, is_hovered)) = query.get_mut(hover_cell) {
                    if pos.eq(&cell) {
                        // 是当前地块hover直接返回
                        return;
                    } else if is_hovered {
                        // is_hovered其实可以省略
                        commands.entity(entity).remove::<MapCellHoveredMarker>();
                        holder.hovered = None;
                        materials.get_mut(material.0.id()).map(|m| {
                            m.color = colors.normal.to_linear();
                        });
                    }
                }
            }

            // 当前hover的MapHexPosition只有在地图内时，才将对应的cell设置为hovered
            if partition.is_valid_position(&cell) {
                let entity = partition.get_cell_by_pos(&cell);
                if let Ok((entity, material, _, _)) = query.get_mut(entity) {
                    commands.entity(entity).insert(MapCellHoveredMarker);
                    holder.hovered = Some(entity);
                    materials.get_mut(material.0.id()).map(|m| {
                        m.color = colors.hovered.to_linear();
                    });
                }
            }
        }
    }
}

// 点击特效系统
pub fn click_effect_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ClickEffect, &mut Transform)>,
) {
    for (entity, mut effect, mut transform) in query.iter_mut() {
        effect.timer.tick(time.delta());
        let progress = effect.timer.fraction();

        // 脉冲动画：先放大后缩小
        let current_scale = if progress < 0.5 {
            lerp(1.0, effect.scale, progress * 2.0)
        } else {
            lerp(effect.scale, 1.0, (progress - 0.5) * 2.0)
        };

        transform.scale = Vec3::splat(current_scale);

        // 完成后自己移除
        if effect.timer.finished() {
            commands.entity(entity).remove::<ClickEffect>();
        }
    }
}

pub fn on_remove_click_effect_system(mut query: Query<&mut Transform, With<HexMapPosition>>) {
    for mut trans in query.iter_mut() {
        trans.scale = Vec3::splat(1.0);
    }
}

// 选中状态系统
pub fn selected_effect_system(
    time: Res<Time>,
    colors: Res<MapCellColors>,
    mut materials: ResMut<Assets<HexagonBorderMaterial>>,
    mut query: Query<(
        &mut MapCellSelectedMarker,
        &mut MeshMaterial2d<HexagonBorderMaterial>,
    )>,
) {
    for (mut selected, material) in query.iter_mut() {
        selected.timer.tick(time.delta());

        // 选中状态闪烁效果
        let blink_factor = (selected.timer.elapsed_secs() * 0.5 * PI).sin().abs();
        let color = colors.selected.to_srgba() * (0.5 + 0.5 * blink_factor);

        materials.get_mut(material.0.id()).map(|m| {
            m.color = color.into();
            m.border_color = Color::srgb(0.00, 1.00, 1.00).into();
            m.border_width = 0.1;
        });
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub struct MapInteractionPlugin;

impl Plugin for MapInteractionPlugin {
    // 在app构建时注册
    fn build(&self, app: &mut App) {
        app.init_resource::<MapCellEffectConfig>()
            .init_resource::<MapCellColors>()
            .insert_resource::<GlobalMousePosition>(GlobalMousePosition::default())
            .insert_resource(SpecialMapCellHolder::default())
            .add_systems(
                Update,
                (
                    update_global_mouse_position_system,
                    (
                        map_cell_click_system.run_if(resource_changed::<ButtonInput<MouseButton>>),
                        map_cell_hover_system,
                        click_effect_system,
                        selected_effect_system,
                        on_remove_click_effect_system.run_if(any_component_removed::<ClickEffect>),
                    ),
                )
                    .in_set(SceneSystemSet::GameSystems)
                    .chain(),
            );
    }
}
