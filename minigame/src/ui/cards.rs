use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::ui::{PositionType, UiRect, Val};

use crate::scenes::scene_selector::SceneSystemSet;

/// 实体卡片组件
#[derive(Component)]
pub struct EntityCard;

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
            .add_systems(
                Startup,
                load_card_assets.in_set(SceneSystemSet::GameSystems),
            )
            .add_systems(
                Update,
                (spawn_entity_card, update_entity_card)
                    .in_set(SceneSystemSet::GameSystems)
                    .chain(),
            ); // 确保按顺序执行
    }
}

fn load_card_assets(asset_server: Res<AssetServer>, mut card_assets: ResMut<CardAssets>) {
    card_assets.font = asset_server.load("fonts/msyh.ttc");
    card_assets.card_bg = asset_server.load("textures/card_bg.png");
}

fn spawn_entity_card(
    mut commands: Commands,
    card_assets: Res<CardAssets>,
    selected_entity: Query<Entity, Added<Selected>>,
    existing_cards: Query<Entity, With<EntityCard>>,
) {
    // 如果卡片已存在，则不创建新卡片
    if !existing_cards.is_empty() {
        return;
    }

    for _ in selected_entity.iter() {
        let card = commands
            .spawn((
                EntityCard,
                Name::new("Entity Info Card"),
                // UI 节点设置
                Node {
                    width: Val::Px(220.0),
                    height: Val::Px(160.0),
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    top: Val::Px(100.0),
                    padding: UiRect::all(Val::Px(15.0)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                // 背景颜色
                BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.95)),
                // 边框设置
                BorderColor(Color::Srgba(Srgba::BLUE)),
                // 层级控制
                GlobalZIndex(10),
            ))
            .id();

        commands.entity(card).with_children(|parent| {
            // 标题
            parent.spawn((
                Name::new("Card Title"),
                Text::new("Entity Info"),
                TextFont {
                    font: card_assets.font.clone(),
                    font_size: 22.0,
                    ..Default::default()
                },
                TextColor(BLUE.into()),
            ));
        });
    }
}

fn update_entity_card(
    mut commands: Commands,
    card_assets: Res<CardAssets>,
    card_query: Query<Entity, With<EntityCard>>,
    selected_entity: Query<&EntityProperties, With<Selected>>,
) {
    // 如果没有卡片实体，直接返回
    let card_entity = if let Ok(entity) = card_query.single() {
        entity
    } else {
        return;
    };

    // 检查选中实体是否存在
    if selected_entity.is_empty() {
        commands.entity(card_entity).despawn();
        return;
    }

    // 获取选中实体的属性
    let props = if let Ok(props) = selected_entity.single() {
        props
    } else {
        commands.entity(card_entity).despawn();
        return;
    };

    // 更新卡片内容
    commands.entity(card_entity).despawn();
    commands.entity(card_entity).with_children(|parent| {
        // 标题
        parent.spawn((
            Name::new("Card Title"),
            Text::new("Entity Info"),
            TextFont {
                font: card_assets.font.clone(),
                font_size: 22.0,
                ..Default::default()
            },
            TextColor(BLUE.into()),
        ));

        // 实体类型
        parent.spawn((
            Name::new("Entity Type"),
            Text::new(format!("Type: {}", props.entity_type)),
            TextFont {
                font: card_assets.font.clone(),
                font_size: 16.0,
                ..Default::default()
            },
            TextColor(WHITE.into()),
        ));

        // 生命值
        parent.spawn((
            Name::new("Entity Health"),
            Text::new(format!("Health: {:.1}", props.health)),
            TextFont {
                font: card_assets.font.clone(),
                font_size: 16.0,
                ..Default::default()
            },
            TextColor(WHITE.into()),
        ));
    });
}

/// 选中标记组件
#[derive(Component)]
pub struct Selected;

/// 实体属性组件
#[derive(Component)]
pub struct EntityProperties {
    pub entity_type: String,
    pub health: f32,
}
