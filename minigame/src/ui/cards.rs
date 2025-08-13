use bevy::prelude::*;
use bevy::ui::{PositionType, Val};

use crate::core::components::EntityType;
use crate::level::config::LevelConfigAsset;
use crate::level::loader::LevelLoader;
use crate::scenes::GameSceneUIRoot;
use crate::scenes::scene_selector::SceneSystemSet;
use crate::sprite::sprite_mgr::SpriteManager;

#[derive(Component)]
struct CardUIRoot;

/// 实体卡片组件
#[derive(Component, Default)]
pub struct EntityCardInfo {
    pub entity_type: EntityType,
    pub cost: u32,
}

// #[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
// pub struct FlowUIMaterial {
//     // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
//     // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
//     #[uniform(0)]
//     color: LinearRgba,
//     // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
//     // add the sampler attribute with a different binding index.
//     #[texture(1)]
//     #[sampler(2)]
//     color_texture: Handle<Image>,
// }

// 选中状态Marker
#[derive(Component)]
pub struct CardSelectedMarker;

#[derive(Resource, Default)]
pub struct SelectedCardHolder(pub(crate) Option<Entity>);

/// 卡片资源
#[derive(Default, Resource)]
pub struct CardAssets {
    pub font: Handle<Font>,
    pub card_bg: Handle<Image>,
}

/// 实体卡片系统插件
pub struct EntityCardsPlugin;

impl Plugin for EntityCardsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardAssets>()
            .insert_resource(SelectedCardHolder::default())
            .add_systems(Startup, load_card_assets)
            .add_systems(
                Update,
                (handle_card_onclick)
                    .in_set(SceneSystemSet::GameSystems)
                    .chain(),
            ); // 确保按顺序执行
    }
}

const SELECTED_BOARDER_COLOR: Color = Color::srgba(0.33, 1.00, 0.50, 0.83);

pub fn spawn_card_ui(
    mut commands: Commands,
    card_assets: Res<CardAssets>,
    level_loader: ResMut<LevelLoader>,
    level_data: Res<Assets<LevelConfigAsset>>,
    ui_root: Query<Entity, With<GameSceneUIRoot>>,
    sprite_manager: Res<SpriteManager>,
    // mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let level_config = level_data.get(&level_loader.level_data).unwrap();
    let parent = ui_root.single().unwrap();

    commands.entity(parent).with_children(|parent| {
        parent
            .spawn((
                Name::new("Cards Root"),
                CardUIRoot,
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(120.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    column_gap: Val::Px(10.0),
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(0.45, 0.45, 0.45, 0.45).into()),
            ))
            .with_children(|root| {
                root.spawn((
                    Name::new("Card"),
                    EntityCardInfo {
                        entity_type: EntityType::Grass,
                        cost: 20,
                    },
                    Interaction::default(),
                    Node {
                        width: Val::Px(68.0),
                        height: Val::Px(100.0),
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip(),
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Px(5.5)),
                    Outline {
                        width: Val::Px(5.),
                        offset: Val::ZERO,
                        color: Color::NONE,
                    },
                    children![(
                        ImageNode::new(card_assets.card_bg.clone()),
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        // MaterialNode(StandardMaterial {
                        //     base_color_texture: Some(card_assets.card_bg.clone()),
                        //     emissive: None,
                        //     ..Default::default()
                        // }),
                        children![
                            (
                                sprite_manager.create_image_node_by_name("grass_normal"),
                                Node {
                                    width: Val::Px(68.),
                                    height: Val::Px(68.),
                                    ..Default::default()
                                }
                            ),
                            (
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Px(40.),
                                    align_content: AlignContent::Center,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                BackgroundColor(Color::srgba(0.184, 0.145, 0.741, 0.781)),
                                BorderRadius::bottom(Val::Px(5.0)),
                                children![(
                                    Text::new("10"),
                                    TextFont {
                                        font: card_assets.font.clone(),
                                        font_size: 28.0,
                                        ..Default::default()
                                    },
                                    TextColor(Color::srgb(1.00, 0.76, 0.76).into()),
                                )]
                            )
                        ]
                    )],
                ));
            });
    });
}

fn load_card_assets(asset_server: Res<AssetServer>, mut card_assets: ResMut<CardAssets>) {
    card_assets.font = asset_server.load("fonts/msyh.ttc");
    card_assets.card_bg = asset_server.load("textures/card_bg.png");
}

fn handle_card_onclick(
    mut commands: Commands,
    mut selected_card: ResMut<SelectedCardHolder>,
    card_query: Query<
        (Entity, Ref<Interaction>, Has<CardSelectedMarker>),
        (With<EntityCardInfo>, Changed<Interaction>),
    >,
    mut outline_query: Query<&mut Outline, With<EntityCardInfo>>,
) {
    for (card_entity, interaction, is_selected) in card_query {
        if interaction.eq(&Interaction::Pressed) {
            if is_selected {
                // 本来就选中了，去除选中效果
                commands.entity(card_entity).remove::<CardSelectedMarker>();
                if let Ok(mut outline) = outline_query.get_mut(card_entity) {
                    outline.color = Color::NONE;
                }
                return;
            }

            if selected_card.0.is_some() {
                let entity = selected_card.0.unwrap();
                commands.entity(entity).remove::<CardSelectedMarker>();
                if let Ok(mut outline) = outline_query.get_mut(entity) {
                    outline.color = Color::NONE;
                }
            }

            selected_card.0 = Some(card_entity);
            if let Ok(mut ol) = outline_query.get_mut(card_entity) {
                ol.color = SELECTED_BOARDER_COLOR;
            }
            commands.entity(card_entity).insert(CardSelectedMarker);
        }
    }
}
