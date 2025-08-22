use std::{collections::HashMap, time::Duration};

use crate::{
    ai::{AnimalActorBoard, EdibleEntity, FrameCounter, Satiety, get_ai_behave_tree},
    core::{
        components::EntityType,
        hex_grid::{HexMapPosition, SpatialPartition},
    },
    level::{
        config::{EntityConfig, LevelConfigAsset},
        loader::LevelLoader,
    },
    scenes::GameSceneRoot,
    sprite::sprite_mgr::SpriteManager,
    ui::{
        BarBorder, BarHeight, BarOrientation, BarSettings, ForegroundColor, PBarColorScheme,
        Percentage, ProgressBarMaterial,
    },
};
use bevy::prelude::*;
use bevy_behave::prelude::BehaveTree;
use bevy_egui::egui::emath::OrderedFloat;

#[derive(Component)]
#[require(Visibility::default())]
pub struct OnMapEntitiesRoot;

/// 基础实体组件
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// 生命状态组件
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

/// 繁殖能力组件
#[derive(Component)]
pub struct Reproduction {
    pub rate: f32,
    pub cooldown: f32,
}

#[derive(Bundle)]
pub struct EntityHeaderBarUI {
    pub sprite: Sprite,
    pub transform: Transform,
}

/// 实体生成器
pub fn spawn_entity(
    commands: &mut Commands,
    config: &EntityConfig,
    sprite_manager: &ResMut<SpriteManager>,
    _bar_mesh: &Res<SharedBarMesh>,
    partition: &mut ResMut<SpatialPartition>,
    parent: &Entity,
) {
    let mut center = partition.grid_to_world(&config.pos);
    center.z = 2.0;

    info!(
        "spawn {:?} at ({:?}, {:?})",
        &config.entity_type, config.pos, center
    );

    let mut cmd = commands.spawn((
        sprite_manager.get_sprite_by_name(config.entity_type.to_string().to_lowercase().as_str()),
        Transform::from_translation(center),
        EdibleEntity::default(),
    ));

    match config.entity_type {
        EntityType::Rabbit => {
            // info!("spawn rabbit behave tree");
            let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
            timer.tick(Duration::from_millis(1500));
            cmd.insert((
                AnimalActorBoard {
                    current_pos: HexMapPosition::from(config.pos),
                    move_cd_timer: timer,
                    entity_type: EntityType::Rabbit,
                    satiety: 5500,
                    decay_faction: 1.1, // TODO根据不同的动物类型配置不同的饱食度衰减
                    ..Default::default()
                },
                children![(
                    Name::new("rabbit behave_tree"),
                    BehaveTree::new(get_ai_behave_tree(EntityType::Rabbit)).with_logging(false),
                )],
            ))
            .with_children(|parent| {
                // parent.spawn((
                //     AnimalStateUIPanel::new(parent.target_entity()),
                //     EntityHeaderBarUI {
                //         sprite: Sprite::from_color(
                //             Color::srgba(0.25, 0.25, 0.55, 0.6),
                //             Vec2::new(100.0, 50.0),
                //         ),
                //         // 这里向上偏移量应当是hell_size + ui_height / 2
                //         transform: Transform::from_translation(
                //             Vec3::Y * partition.config.size + Vec3::Z * 3.0,
                //         ),
                //     },
                //     // ProgressBar
                // ));

                // stomach icon
                parent.spawn((
                    Sprite {
                        image: sprite_manager.stomach_icon.clone(),
                        custom_size: Some(Vec2::splat(20.)),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(
                        -partition.config.size / 2.,
                        -partition.config.size / 2.,
                        3.0,
                    )),
                ));

                parent.spawn((
                    Satiety(5500),
                    BarSettings::<Satiety> {
                        width: partition.config.size * 0.7,
                        offset: -partition.config.size / 2.,
                        height: BarHeight::Static(10.),
                        orientation: BarOrientation::Vertical,
                        border: BarBorder::new(2.0),
                        ..Default::default()
                    },
                ));
            });
        }
        EntityType::Fox => {
            // cmd.insert(Fox);
        }
        _ => {}
    };

    cmd.insert(ChildOf(*parent));

    partition.insert_cache_entity(cmd.id(), &config.pos.into(), config.entity_type.clone());
}

pub fn spawn_satiety_pbar_onadd(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_handles: ResMut<MeshHandles>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
    // mut materials2: ResMut<Assets<ColorMaterial>>,
    color_scheme: Res<PBarColorScheme<Satiety>>,
    query: Query<(Entity, &Satiety, &BarSettings<Satiety>), Added<Satiety>>,
) {
    for (entity, satiety, settings) in query.iter() {
        let width = settings.normalized_width();
        let height = settings.normalized_height();

        let mesh = mesh_handles.get(width, height).unwrap_or_else(|| {
            mesh_handles.insert(
                width,
                height,
                meshes.add(Mesh::from(Rectangle::new(width, height))),
            )
        });

        let (high, moderate, low) = match color_scheme.foreground_color {
            ForegroundColor::Static(color) => (color, color, color),
            ForegroundColor::TriSpectrum {
                high,
                moderate,
                low,
            } => (high, moderate, low),
        };
        info!(
            "=====satiety width: {}, height: {}, settings: {:#?}",
            width, height, settings
        );

        // satiety bar
        commands.entity(entity).insert((
            Mesh2d(mesh.0),
            Transform::from_translation(Vec3::ZERO),
            MeshMaterial2d(materials.add(ProgressBarMaterial {
                value_and_dimensions:
                    (satiety.value(), width, height, settings.border.width).into(),
                background_color: color_scheme.background_color.to_linear(),
                high_color: high.to_linear(),
                moderate_color: moderate.to_linear(),
                low_color: low.to_linear(),
                offset: settings.normalized_offset().extend(0.),
                border_color: Color::srgba(0.00, 0.00, 0.00, 0.35).to_linear(),
                vertical: settings.orientation == BarOrientation::Vertical,
            })),
            GlobalZIndex(5),
        ));
    }
}

#[derive(Resource, Default)]
pub struct MeshHandles(pub HashMap<(OrderedFloat<f32>, OrderedFloat<f32>), Handle<Mesh>>);

impl MeshHandles {
    pub fn get(&self, width: f32, height: f32) -> Option<Mesh2d> {
        self.0
            .get(&(OrderedFloat(width), OrderedFloat(height)))
            .cloned()
            .map(Mesh2d)
    }

    pub fn insert(&mut self, width: f32, height: f32, handle: Handle<Mesh>) -> Mesh2d {
        self.0
            .insert((OrderedFloat(width), OrderedFloat(height)), handle.clone());

        Mesh2d(handle)
    }
}

#[derive(Resource)]
pub struct SharedBarMesh(pub Handle<Mesh>);
pub fn pre_spawn_entities_system(
    mut commands: Commands,
    mut meshed: ResMut<Assets<Mesh>>,
    partition: Res<SpatialPartition>,
) {
    commands.insert_resource(SharedBarMesh(
        meshed.add(Rectangle::new(5., partition.config.size * 1.5)),
    ));
}

pub fn spawn_entities_system(
    mut commands: Commands,
    level_loader: Res<LevelLoader>,
    level_data: Res<Assets<LevelConfigAsset>>,
    sprite_manager: ResMut<SpriteManager>,
    mut partition: ResMut<SpatialPartition>,
    bar_mesh: Res<SharedBarMesh>,
    root: Query<Entity, With<GameSceneRoot>>,
) {
    commands.insert_resource(FrameCounter::default());
    let level_config = level_data.get(&level_loader.level_data).unwrap();

    let root_parent = root.single().unwrap();

    let parent = commands
        .spawn((OnMapEntitiesRoot, Transform::from_xyz(0.0, 0.0, 2.0)))
        .insert(ChildOf(root_parent))
        .id();

    for cfg in level_config.entities.iter() {
        spawn_entity(
            &mut commands,
            cfg,
            &sprite_manager,
            &bar_mesh,
            &mut partition,
            &parent,
        );
    }
}
