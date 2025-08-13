use crate::core::components::EntityType;
use crate::core::hex_grid::{EntityWithCoord, HexMapPosition, hex_distance};
use crate::core::systems::hex_grid::SpatialPartition;
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy_behave::prelude::*;
use pathfinding::prelude::*;
use std::cmp::min;

// 探索方向（随机移动）偏好组件
#[derive(Component, Debug, Clone)]
pub struct MovementPreference {
    pub direction: Vec3, // 当前偏好方向向量，以此direction为ZERO表示未初始化探索方向
    pub strength: f32,   // 方向偏好强度 (0.0-1.0)
    pub stability: f32,  // 方向稳定性 (0-1, 越高越不易改变)
}

impl Default for MovementPreference {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO, // 默认向右
            strength: 0.7,
            stability: 0.8,
        }
    }
}

// 记录一次探索过程数据的组件
#[derive(Component, Debug, Default, Clone)]
pub struct Exploration {
    pub direction: Vec3,               // 当前探索方向
    pub steps_remaining: i8,           // 剩余移动步数
    pub base_position: HexMapPosition, // 探索起始点
}

#[derive(Component, Debug, Clone, Default)]
pub struct ForageAction {
    pub food_entity_type: EntityType,
}

#[derive(Component, Debug, Default, Clone)]
pub struct IdleAction {
    pub preference: MovementPreference,
    pub exploration: Exploration,
}

pub fn get_ai_behave_tree(entity_type: EntityType) -> Tree<Behave> {
    let forage_subtree = behave! {
        Behave::Fallback => {
            Behave::spawn_named("Forage Action", ForageAction { food_entity_type: EntityType::Grass}),
        }
    };
    let flee_subtree = behave! {
        Behave::Fallback => {
            Behave::AlwaysFail,
        }
    };
    let idle_subtree = behave! {
        Behave::Fallback => {
            Behave::spawn_named("Idle Action", IdleAction::default()),
        }
    };

    match entity_type {
        EntityType::Rabbit => {
            return behave! {
                Behave::Forever => {
                    Behave::Fallback => {
                        @ flee_subtree,
                        @ forage_subtree,
                        @ idle_subtree
                    }
                }
            };
        }
        EntityType::Fox => {
            return behave! {
                Behave::Forever => {
                    Behave::Fallback => {
                        @forage_subtree,
                        @idle_subtree
                    }
                }
            };
        }
        _ => {
            behave! {
                Behave::AlwaysSucceed => {
                    @idle_subtree
                }
            }
        }
    }
}

// 记录可食用实体被标记为预占用的情况，避免觅食竞争
#[derive(Component, Debug, Clone, Default)]
pub struct EdibleEntity {
    pub reserved_by: Option<Entity>,
}

#[derive(Component, Debug, Clone, Default, PartialEq)]
pub enum ActorState {
    #[default]
    Idle,
    RandomMove,
    Foraging,
    Flee,
}
#[derive(Component, Debug, Clone, Default)]
#[require(Transform)]
pub struct AnimalActorBoard {
    pub current_pos: HexMapPosition,         // 当前六边形坐标
    pub forage_target: Option<Entity>,       // 觅食对象
    pub move_target: Option<HexMapPosition>, // 移动目标坐标
    pub path_buffer: Vec<HexMapPosition>,    // 预计算的路径缓冲区
    pub state: ActorState,                   // 当前行为状态
    pub idle_counter: u32,                   // 空闲计数器
    pub move_cd_timer: Timer,                // 移动CD计时器
    pub satiety: i32,                        // 饱食度:放大100倍来避免float类型计算
    pub decay_faction: f32,                  // 饱食度衰减因子，表示每秒衰减的饱食度
    pub path_cost: f32,                      // 路径代价（用于D*Lite）[2](@ref)
    pub entity_type: EntityType,
}

impl AnimalActorBoard {
    pub fn set_forage_target(&mut self, target: EntityWithCoord) {
        self.forage_target = Some(target.entity);
        self.move_target = Some(target.pos);
    }

    pub fn clear_forage_target(&mut self) {
        self.forage_target = None;
        self.move_target = None;
    }

    pub fn do_eat(&mut self) -> EntityWithCoord {
        let result = EntityWithCoord {
            entity: self.forage_target.unwrap(),
            pos: self.move_target.unwrap(),
        };
        self.satiety += 5000;
        self.clear_forage_target();
        return result;
    }
}

// // 新增寻路请求队列（异步处理避免卡顿）
// #[derive(Resource, Default)]
// pub struct PathfindingQueue {
//     requests: Vec<(Entity, UVec2, PathAlgorithm)>, // (实体, 目标点, 算法类型)
// }

// 算法选择策略
#[derive(Debug, Clone, Copy)]
pub enum PathAlgorithm {
    DStarLite, // 长CD动物
    APF,       // 短CD动物（人工势场法）
    Hybrid,    // 中CD动物（混合策略）
}

pub fn forage_action_system(
    mut commands: Commands,
    mut query: Query<(&BehaveCtx, &mut ForageAction)>,
    mut actor_query: Query<(&mut Transform, &mut AnimalActorBoard)>,
    mut target_query: Query<(Entity, &mut EdibleEntity)>,
    mut partition: ResMut<SpatialPartition>,
    time: Res<Time>,
) {
    for (ctx, action) in query.iter_mut() {
        let this_entity = ctx.target_entity();
        if let Ok((mut transform, mut actor)) = actor_query.get_mut(this_entity) {
            match actor.state {
                ActorState::Flee => {
                    // 检查状态，如果是Flee状态则退出觅食逻辑
                    commands.trigger(ctx.failure());
                    continue;
                }
                _ => {
                    if actor.satiety >= 8000 {
                        actor.state = ActorState::Idle;
                        actor.clear_forage_target();
                        commands.trigger(ctx.failure());
                        continue;
                    } else {
                        actor.state = ActorState::Foraging;
                    }
                }
            }

            // 移动Cd未冷却时不能行动
            if !actor.move_cd_timer.tick(time.delta()).finished() {
                continue;
            }

            actor.move_cd_timer.reset();
            // 对于已经有目标的要检查目标的预占对象是否是自己
            if let Some(target) = actor.forage_target {
                if let Ok((_, edible)) = target_query.get(target) {
                    if let Some(reserved) = edible.reserved_by
                        && reserved != this_entity
                    {
                        actor.clear_forage_target();
                    }
                }
            }

            // 选定觅食目标 或 重新选定觅食目标
            if actor.forage_target.is_none() {
                // let food_type = match actor.entity_type {
                //     EntityType::Rabbit => EntityType::Grass,
                //     EntityType::Fox => EntityType::Rabbit,
                //     _ => panic!("Unknown forage type"),
                // };
                let food_type = action.food_entity_type.clone();
                // actor.current_pos
                let mut entities = partition.entities_by_type(&food_type);
                if entities.is_empty() {
                    warn!("forage_action: No {food_type:?} to forage!");
                    actor.clear_forage_target();
                    continue;
                }

                // 根据距离排序
                entities.sort_by(|a, b| {
                    hex_distance(&a.pos, &actor.current_pos)
                        .cmp(&hex_distance(&b.pos, &actor.current_pos))
                });
                info!("sorted entities: {entities:?}");

                for e in entities {
                    if let Ok((_, mut edible)) = target_query.get_mut(e.entity) {
                        if edible.reserved_by.is_none() {
                            warn!("forage_action: get forage {e:?}");
                            edible.reserved_by = Some(this_entity.clone());
                            actor.set_forage_target(e);
                            break;
                        }
                    }
                }
            }

            // 有觅食目标的时候，向目标移动
            if let Some(move_target) = actor.move_target {
                let pref = partition.as_ref();
                // 首先处理觅食者就站在食物上的情况
                if actor.current_pos.eq(&move_target) {
                    do_eat_and_despawn_food_entity(
                        &mut commands,
                        &target_query,
                        &mut partition,
                        action.food_entity_type.clone(),
                        &mut actor,
                    );
                    commands.trigger(ctx.failure());
                    continue;
                }

                if let Some((path, _)) = astar(
                    &actor.current_pos,
                    |p| {
                        pref.get_valid_neighbours(p)
                            .into_iter()
                            .map(|p| (p, 1))
                            .collect::<Vec<_>>()
                    },
                    |p| hex_distance(p, &move_target),
                    |p| *p == move_target,
                ) {
                    info!("forage action: Path to forage target: {path:?}");
                    if path.len() > 1 {
                        //move to
                        let next_pos = path[1];
                        move_actor_to_next_pos(
                            &mut actor,
                            &mut transform,
                            &next_pos,
                            &mut partition,
                        );
                        if next_pos == move_target {
                            do_eat_and_despawn_food_entity(
                                &mut commands,
                                &target_query,
                                &mut partition,
                                action.food_entity_type.clone(),
                                &mut actor,
                            );
                            commands.trigger(ctx.failure());
                            continue;
                        }
                    }
                }
            }
        }
    }
}

// 执行吃掉食物并清理食物实体的逻辑
fn do_eat_and_despawn_food_entity(
    commands: &mut Commands,
    target_query: &Query<(Entity, &mut EdibleEntity)>,
    partition: &mut SpatialPartition,
    food_type: EntityType,
    actor: &mut AnimalActorBoard,
) {
    // 修正AnimalActorBoard的数据
    let food = actor.do_eat();
    // 从SpatialPartition移除食物，移除实体的时候要先把数据从SpatialPartition中移除，才能移除实体。
    partition.remove_entity(food.entity, &food.pos, food_type);
    // 销毁食物对应的实体
    if let Ok((entity, _)) = target_query.get(food.entity) {
        commands.entity(entity).despawn();
    }
}

pub fn idle_action_system(
    mut commands: Commands,
    mut query: Query<(&BehaveCtx, &mut IdleAction)>,
    mut board_query: Query<(&mut Transform, &mut AnimalActorBoard)>,
    mut partition: ResMut<SpatialPartition>,
    time: Res<Time>,
) {
    for (ctx, mut action) in query.iter_mut() {
        if let Ok((mut transform, mut actor)) = board_query.get_mut(ctx.target_entity()) {
            // TODO 如果进入食物链捕食者视野范围，则进入逃离模式
            // 如果进入饥饿临界值，进入觅食状态
            if actor.satiety <= 5000 {
                actor.state = ActorState::Foraging;
                commands.trigger(ctx.failure());
                continue;
            }

            // 移动cd未结束时不进行行动
            if !actor.move_cd_timer.tick(time.delta()).finished() {
                continue;
            }

            actor.move_cd_timer.reset();

            match actor.state {
                ActorState::Idle => {
                    actor.idle_counter += 1;
                    // 保持Idle的概率：最大30%的概率，idle_counter每增加1，概率降低10%，但最小保留1%的概率。
                    if rand::random_ratio(min(99, 50 + actor.idle_counter * 10), 100) {
                        actor.state = ActorState::RandomMove;
                        actor.idle_counter = 0;
                        // 将探索方向设置为ZERO在下面的逻辑中进行初始化
                        action.preference.direction = Vec3::ZERO;
                    } else {
                        warn!("idle_action: idle...");
                        continue;
                    }
                }
                ActorState::RandomMove => {}
                _ => {
                    commands.trigger(ctx.failure());
                    continue;
                }
            }

            // 运行到此处，actor处于RandomMove状态
            // 判断一个坐标对应的地块是否可以通行
            let is_valid = |cell: &HexMapPosition| {
                partition.is_valid_position(cell) && !partition.is_obstacle(cell)
            };
            // 获取周边可以同行的地块集合
            let neighbours: Vec<HexMapPosition> = partition
                .get_valid_neighbours(&actor.current_pos.into())
                .into_iter()
                .filter(is_valid)
                .collect();
            // 如果周围都不可通行，则随机漫步失败。
            if neighbours.is_empty() {
                info!("random_walk failed! target: {:?}", actor.current_pos);
                commands.trigger(ctx.failure()); //TODO 无法移动时直接失败可能对性能有影响
                continue;
            }

            // 检查MovePreference的随机方向，若方向没有改变，则重新随机生成方向向量
            if action.preference.direction == Vec3::ZERO {
                let target = if neighbours.len() > 1 {
                    neighbours[rand::random_range(1..neighbours.len()) - 1]
                } else {
                    neighbours[0]
                };
                action.preference.direction = (target.cube_coord()
                    - actor.current_pos.cube_coord())
                .as_vec3()
                .normalize();
                // 初始化探索行为数据
                action.exploration = Exploration {
                    direction: action.preference.direction,
                    steps_remaining: rand::random_range(3..6),
                    base_position: actor.current_pos,
                };

                // partition.entity_move()
                move_actor_to_next_pos(&mut actor, &mut transform, &target, &mut partition);
            } else {
                // 对于有配置的，则进行随机偏移
                action.exploration.steps_remaining -= 1;
                // 完成本次探索后回归Idle状态
                if action.exploration.steps_remaining < 0 {
                    actor.state = ActorState::Idle;
                    actor.idle_counter = 0;
                    info!("exploration finished.");
                    continue;
                }

                let mut candidates = Vec::new();

                // info!("Neighbours: {:?}", neighbours);
                for neighbour in neighbours {
                    // 1. 方向偏好评分
                    let dir_score = {
                        let vec_to_neighbour =
                            neighbour.cube_coord() - actor.current_pos.cube_coord();
                        let pref_dir = action.preference.direction.normalize();
                        vec_to_neighbour.as_vec3().normalize().dot(pref_dir)
                    };

                    // 2. 动态避障检测
                    let collision_risk =
                        dynamic_obstacle_risk(&neighbour, &partition, ctx.target_entity());

                    // 3. 综合权重 = 基础权重 + 方向偏好 - 避障惩罚
                    let weight =
                        1.0 + (dir_score * action.preference.strength) - (collision_risk * 10.0);

                    candidates.push((neighbour, weight.max(0.01))); // 最小权重避免除零
                }
                if let Some(target_pos) = weighted_random_choice(&candidates) {
                    info!("Next Move To: {:?}", &target_pos);
                    move_actor_to_next_pos(&mut actor, &mut transform, &target_pos, &mut partition);
                }
            }
        }
    }
}

fn move_actor_to_next_pos(
    actor: &mut AnimalActorBoard,
    transform: &mut Transform,
    target_pos: &HexMapPosition,
    partition: &mut SpatialPartition,
) {
    //TODO entity移动了，需要修改partition中的entities的数据
    actor.current_pos = target_pos.clone();
    transform.translation = partition.grid_to_world(&target_pos.to_vec2()) + Vec3::Z * 3.0;
}

pub fn onadd_idle_action(
    _trigger: Trigger<OnAdd, IdleAction>,
    _q: Query<&BehaveCtx, With<IdleAction>>,
) {
    info!("==IdleAction:OnAdd==");
}

// 动态避障检测函数
fn dynamic_obstacle_risk(
    target: &HexMapPosition,
    partition: &SpatialPartition,
    exclude_entity: Entity,
) -> f32 {
    let mut risk: f32 = 0.0;

    // 1. 检查目标位置是否有其他实体
    let entities = partition.entities_at(target);
    for entity in entities {
        if entity != exclude_entity {
            risk = risk.max(1.0); // 完全避免碰撞
        }
    }

    // 2. 检查相邻位置的实体密度
    for neighbour in partition.get_valid_neighbours(target) {
        let entities = partition.entities_at(&neighbour);
        let count = entities
            .into_iter()
            .filter(|&e| e != exclude_entity)
            .count() as f32;
        risk = risk.max(count * 0.3);
    }

    risk
}

// 基于权重的概率选择（Rust版）
fn weighted_random_choice(candidates: &[(HexMapPosition, f32)]) -> Option<HexMapPosition> {
    if candidates.is_empty() {
        return None;
    }

    // 1. 计算总权重
    let total_weight: f32 = candidates.iter().map(|(_, w)| w).sum();

    // 2. 生成[0, total_weight]区间随机数
    let mut rand_val = rand::random_range(0.0..=total_weight);

    // 3. 遍历找到权重区间匹配项
    for (pos, weight) in candidates {
        rand_val -= weight;
        if rand_val <= 0.0 {
            return Some(*pos);
        }
    }

    // 4. 浮点误差处理
    candidates.last().map(|(p, _)| *p)
}

pub fn render_gizmos(
    mut gizmos: Gizmos,
    query: Query<(&BehaveCtx, &mut IdleAction)>,
    board_query: Query<(&Transform, &AnimalActorBoard)>,
    partition: Res<SpatialPartition>,
) {
    for (ctx, action) in query {
        if let Ok((trans, _)) = board_query.get(ctx.target_entity()) {
            let end = trans.translation + action.preference.direction * 50.0;
            gizmos.arrow_2d(trans.translation.xy(), end.xy(), RED);
        }
    }
    for (transform, board) in board_query {
        let location = transform.translation.xy();
        if board.satiety <= 5000 {
            gizmos.circle_2d(location.clone(), 30.0, RED);
        }

        if let Some(target) = board.move_target {
            let end = partition.grid_to_world(&target.to_vec2());
            gizmos.line_2d(location, end.xy(), RED);
        }
    }
}
