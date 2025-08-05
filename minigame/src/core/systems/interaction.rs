use crate::core::systems::hex_grid::{HexMapPosition, HexagonBorderMaterial};
use crate::scenes::scene_selector::SceneSystemSet;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct GlobalMousePosition {
    pub is_in_primary_window: bool,
    pub pos: Vec2,
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
pub struct Selected {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Hovered;

#[derive(Component)]
pub struct ClickEffect {
    pub timer: Timer,
    pub scale: f32,
}

// 交互配置资源
#[derive(Resource)]
pub struct HexInteraction {
    pub click_radius: f32,
    pub hover_radius: f32,
    pub click_effect_duration: f32,
    pub click_effect_scale: f32,
    pub particle_speed: f32,
    pub particle_count: u32,
    pub animation_curve: AnimationCurve,
}

impl Default for HexInteraction {
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
pub struct HexColors {
    pub normal: Color,
    pub hovered: Color,
    pub selected: Color,
    pub click_effect: Color,
}

impl Default for HexColors {
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
pub fn hex_click_system(
    interaction: Res<HexInteraction>,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_position: Res<GlobalMousePosition>,
    query: Query<(Entity, &Transform, &HexMapPosition)>,
    mut commands: Commands,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    if !mouse_position.is_in_primary_window {
        return;
    }

    let cursor_pos = mouse_position.pos;

    for (entity, transform, _) in query.iter() {
        let distance = transform.translation.truncate().distance(cursor_pos);
        if distance <= interaction.click_radius {
            commands
                .entity(entity)
                .insert(Selected {
                    timer: Timer::from_seconds(0.1, TimerMode::Once),
                })
                .insert(ClickEffect {
                    timer: Timer::from_seconds(interaction.click_effect_duration, TimerMode::Once),
                    scale: interaction.click_effect_scale,
                });
            break;
        }
    }
}

// 悬停检测系统, 遍历所有的地图块效率太低。应当建立地图块的图元信息二维数组，进行精准定位计算
pub fn hex_hover_system(
    // mut commands: Commands,
    interaction: Res<HexInteraction>,
    mouse_position: Res<GlobalMousePosition>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<
        (Entity, &Transform, &MeshMaterial2d<HexagonBorderMaterial>),
        With<HexMapPosition>,
    >,
    colors: Res<HexColors>,
    mut materials: ResMut<Assets<HexagonBorderMaterial>>,
) {
    if !mouse_position.is_in_primary_window {
        return;
    }
    let cursor_pos = mouse_position.pos;
    if let Ok((camera, cam_transform)) = camera_q.single() {
        if let Ok(pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) {
            for (_entity, transform, material) in query.iter_mut() {
                let distance = transform.translation.truncate().distance(pos);
                let is_hovered = distance <= interaction.hover_radius;

                if is_hovered {
                    // commands.entity(entity).insert(Hovered);
                    materials.get_mut(material.0.id()).map(|m| {
                        m.color = colors.hovered.to_linear();
                    });
                } else {
                    // // commands.entity(entity).remove::<Hovered>();
                    materials.get_mut(material.0.id()).map(|m| {
                        m.color = colors.normal.to_linear();
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
    colors: Res<HexColors>,
    mut materials: ResMut<Assets<HexagonBorderMaterial>>,
    mut query: Query<(&mut Selected, &mut MeshMaterial2d<HexagonBorderMaterial>)>,
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
        app.init_resource::<HexInteraction>()
            .init_resource::<HexColors>()
            .insert_resource::<GlobalMousePosition>(GlobalMousePosition::default())
            .add_systems(
                Update,
                (
                    update_global_mouse_position_system,
                    (
                        hex_click_system,
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
