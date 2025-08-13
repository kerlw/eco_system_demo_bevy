use crate::core::hex_grid::SpatialPartition;
use crate::core::systems::hex_grid::{HexMapPosition, HexagonBorderMaterial};
use crate::scenes::scene_selector::SceneSystemSet;
use crate::ui::SelectedCardHolder;
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::ecs::resource;
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
            click_effect_duration: 0.3,
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
            normal: Color::srgb(0.2, 0.2, 0.8),
            hovered: Color::srgb(0.3, 0.3, 0.9),
            selected: Color::srgb(0.8, 0.2, 0.2),
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
pub fn map_cell_click_system(
    interaction: Res<MapCellEffectConfig>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_position: Res<GlobalMousePosition>,
    query: Query<(Entity, &Transform, &HexMapPosition)>,
    mut cell_holder: ResMut<SpecialMapCellHolder>,
    card_holder: Res<SelectedCardHolder>,
    partition: Res<SpatialPartition>,
    mut commands: Commands,
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
            let cell = partition.world_to_grid(&cursor_pos);
            // 地块坐标在地图范围内
            if partition.is_valid_position(&cell) {
                if let Some(card) = selected_card { // 选择了卡片，则处理投放
                } else { // 未选择卡片则处理选中地块
                }
            } else if selected_card.is_none() { // 不在地图范围内时，仅处理未选中卡片的情况，取消已经设置了selected的地块组件
            }
        }
    }

    // for (entity, transform, _) in query.iter() {
    //     let distance = transform.translation.truncate().distance(cursor_pos);
    //     if distance <= interaction.click_radius {
    //         commands
    //             .entity(entity)
    //             .insert(MapCellSelectedMarker {
    //                 timer: Timer::from_seconds(0.1, TimerMode::Once),
    //             })
    //             .insert(ClickEffect {
    //                 timer: Timer::from_seconds(interaction.click_effect_duration, TimerMode::Once),
    //                 scale: interaction.click_effect_scale,
    //             });
    //         break;
    //     }
    // }
}

// 悬停检测系统, 遍历所有的地图块效率太低。应当建立地图块的图元信息二维数组，进行精准定位计算
pub fn hex_hover_system(
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

        if effect.timer.finished() {
            commands.entity(entity).remove::<ClickEffect>();
        }
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
        info!("selected_effect_system");
        selected.timer.tick(time.delta());

        // 选中状态闪烁效果
        let blink_factor = (selected.timer.elapsed_secs() * 5.0).sin().abs();
        let color = colors.selected.to_srgba() * (0.7 + 0.3 * blink_factor);

        materials.get_mut(material.0.id()).map(|m| {
            m.color = color.into();
            // border_color: AQUA.into(),
            // border_width: 0.1,
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
                        hex_hover_system,
                        click_effect_system,
                        selected_effect_system,
                    ),
                )
                    .in_set(SceneSystemSet::GameSystems)
                    .chain(),
            );
    }
}
