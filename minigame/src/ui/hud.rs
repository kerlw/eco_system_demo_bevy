use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::ui::{FlexDirection, PositionType, UiRect, Val};

use crate::scenes::scene_selector::SceneSystemSet;

/// HUD根节点组件标记
#[derive(Component)]
pub struct HudRoot;

/// 分数文本组件
#[derive(Component)]
pub struct ScoreText;

/// 时间文本组件
#[derive(Component)]
pub struct TimeText;

/// HUD资源
#[derive(Default, Resource)]
pub struct HudAssets {
    pub font: Handle<Font>,
}

/// HUD界面系统插件
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HudAssets>()
            .add_systems(Startup, setup_hud.in_set(SceneSystemSet::GameSystems))
            .add_systems(
                Update,
                (update_score_text, update_time_text).in_set(SceneSystemSet::GameSystems),
            );
    }
}

fn setup_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut hud_assets: ResMut<HudAssets>,
) {
    // 加载字体资源
    hud_assets.font = asset_server.load("fonts/msyhbd.ttc");

    // 创建HUD根节点
    commands
        .spawn((
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(15.0),
                position_type: PositionType::Absolute,
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.7).into()),
            GlobalZIndex(1), // 确保在最上层
        ))
        .with_children(|parent| {
            // 分数文本
            parent.spawn((
                ScoreText,
                Text::new("Score: 0"),
                TextFont {
                    font: hud_assets.font.clone(),
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor(WHITE.into()),
            ));

            // 时间文本
            parent.spawn((
                TimeText,
                Text::new("Time: 0s"),
                TextFont {
                    font: hud_assets.font.clone(),
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor(WHITE.into()),
                // UiRect::left(Val::Px(20.0)),
                // .with_style(Style {
                //     margin: UiRect::left(Val::Px(20.0)),
                //     ..Default::default()
                // }),
            ));
        });
}

fn update_score_text(
    mut query: Query<&mut Text, With<ScoreText>>,
    // 添加你的分数查询逻辑
) {
    for mut text in &mut query {
        // 更新分数文本
        // 替换为实际分数逻辑
        *text = Text::new(format!("Score: {}", 0));
    }
}

fn update_time_text(mut query: Query<&mut Text, With<TimeText>>, time: Res<Time>) {
    for mut text in &mut query {
        // 更新时间文本
        *text = Text::new(format!("Time: {:.1}s", time.elapsed_secs()));
    }
}
