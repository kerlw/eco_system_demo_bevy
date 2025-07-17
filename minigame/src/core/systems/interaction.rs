use crate::core::systems::hex_grid::HexCell;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

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
            click_radius: 50.0,
            hover_radius: 55.0,
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

// 点击检测系统
pub fn hex_click_system(
    interaction: Res<HexInteraction>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut cursor_events: EventReader<CursorMoved>,
    query: Query<(Entity, &Transform, &HexCell)>,
    mut commands: Commands,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_pos) = cursor_events.read().last().and_then(|w| Some(w.position)) else {
        return;
    };

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

// 悬停检测系统
pub fn hex_hover_system(
    mut commands: Commands,
    interaction: Res<HexInteraction>,
    mut cursor_events: EventReader<CursorMoved>,
    mut query: Query<(Entity, &Transform, &mut MeshMaterial2d<ColorMaterial>), With<HexCell>>,
    colors: Res<HexColors>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Some(cursor_pos) = cursor_events.read().last().and_then(|w| Some(w.position)) else {
        return;
    };

    for (entity, transform, mut material) in query.iter_mut() {
        let distance = transform.translation.truncate().distance(cursor_pos);
        let is_hovered = distance <= interaction.hover_radius;

        if is_hovered {
            commands.entity(entity).insert(Hovered);
            *material = MeshMaterial2d(materials.add(colors.hovered));
        } else {
            commands.entity(entity).remove::<Hovered>();
            *material = MeshMaterial2d(materials.add(colors.normal));
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
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Selected, &mut MeshMaterial2d<ColorMaterial>)>,
) {
    for (mut selected, mut material) in query.iter_mut() {
        selected.timer.tick(time.delta());

        // 选中状态闪烁效果
        let blink_factor = (selected.timer.elapsed_secs() * 5.0).sin().abs();
        let color = colors.selected.to_srgba() * (0.7 + 0.3 * blink_factor);

        *material = MeshMaterial2d(materials.add(Color::Srgba(color)));
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
            .add_systems(
                Update,
                (
                    hex_click_system,
                    hex_hover_system,
                    click_effect_system,
                    selected_effect_system,
                ),
            );
    }
}
