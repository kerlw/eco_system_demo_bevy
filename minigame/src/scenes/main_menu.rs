use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::ui::{FlexDirection, UiRect, Val};
use std::fs;

use crate::core::GameState;
use crate::level::loader::LevelLoader;

/// 关卡选择UI根节点组件标记
#[derive(Component)]
pub struct MainMenuRoot;

/// 关卡按钮组件
#[derive(Component)]
pub struct LevelButton {
    pub level_name: String,
}

/// 关卡选择UI系统插件
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// 设置关卡选择UI
pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/msyhbd.ttc");
    // 创建关卡选择UI根节点
    commands
        .spawn((
            Camera2d::default(),
            MainMenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.08, 0.15, 0.40)),
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new("选择关卡"),
                TextFont {
                    font: font.clone(),
                    font_size: 48.0,
                    ..Default::default()
                },
                TextColor(WHITE.into()),
            ));

            // 关卡按钮容器
            parent
                .spawn((
                    Node {
                        width: Val::Percent(80.0),
                        height: Val::Percent(60.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.15, 0.15, 0.9)),
                ))
                .with_children(|parent| {
                    // 枚举assets/levels目录下的所有关卡文件
                    if let Ok(entries) = fs::read_dir("assets/levels") {
                        for entry in entries.flatten() {
                            info!(
                                "display:{} filename:{:?}",
                                entry.path().display(),
                                entry.file_name()
                            );
                            if let Some(level_name) = entry.file_name().to_str() {
                                if level_name.ends_with(".lvc") {
                                    let level_name = level_name.replace(".lvc", "");

                                    // 创建关卡按钮
                                    parent.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(300.0),
                                            height: Val::Px(120.0),
                                            border: UiRect::all(Val::Px(5.0)),
                                            // horizontally center child text
                                            justify_content: JustifyContent::Center,
                                            // vertically center child text
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BorderColor(Color::BLACK),
                                        BorderRadius::MAX,
                                        BackgroundColor(NORMAL_BUTTON),
                                        LevelButton {
                                            level_name: level_name.clone(),
                                        },
                                        children![(
                                            Text::new(level_name),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 24.0,
                                                ..Default::default()
                                            },
                                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                            TextShadow::default(),
                                        )],
                                    ));
                                }
                            }
                        }
                    }
                    info!("here!");
                });
        });
}

/// 处理关卡按钮交互
pub fn handle_level_button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &LevelButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut level_loader: ResMut<LevelLoader>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut boarder_color, level_button) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                boarder_color.0 = WHITE.into();
            }
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                boarder_color.0 = RED.into();
                // 设置当前关卡
                level_loader.current_level = Some(level_button.level_name.clone());
                level_loader.loading = false;
                // 切换到游戏场景
                game_state.set(GameState::GameLoading);
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                boarder_color.0 = Color::BLACK;
            }
        }
    }
}

// 移除关卡选择UI
pub fn despawn_level_selection_ui(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuRoot>>,
) {
    if let Ok(root) = query.single() {
        commands.entity(root).despawn();
    }
}
