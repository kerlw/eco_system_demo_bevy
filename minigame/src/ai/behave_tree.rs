use std::cmp::max;

use bevy::prelude::*;
use bevy_behave::prelude::*;

use crate::core::components::EntityType;
use crate::core::hex_grid::get_neighbours;
use crate::core::systems::hex_grid::SpatialPartition;

#[derive(Component, Default, Clone)]
pub struct IdleAction;

pub fn get_ai_behave_tree(entity_type: EntityType) -> Tree<Behave> {
    let eat_subtree = behave! {
        Behave::Fallback => {
            Behave::AlwaysFail,
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
                        @ eat_subtree,
                        @ idle_subtree
                    }
                }
            };
        }
        EntityType::Fox => {
            return behave! {
                Behave::Forever => {
                    Behave::Fallback => {
                        @eat_subtree,
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

#[derive(Component, Debug, Clone, Default)]
pub enum ActorState {
    #[default]
    Idle,
    RandomMove,
    Seeking,
    Flee,
}
#[derive(Component, Debug, Clone, Default)]
#[require(Transform)]
pub struct AnimalActorBoard {
    pub current_pos: IVec2,         // 当前六边形坐标
    pub move_target: Option<IVec2>, // 移动目标坐标
    pub path_buffer: Vec<IVec2>,    // 预计算的路径缓冲区
    pub state: ActorState,          // 当前行为状态
    pub idle_counter: u32,          // 空闲计数器
    pub move_cd_timer: Timer,       // 移动CD计时器
    pub satiety: u32,               // 饱食度
    pub path_cost: f32,             // 路径代价（用于D*Lite）[2](@ref)
    pub entity_type: EntityType,
}

// 新增寻路请求队列（异步处理避免卡顿）
#[derive(Resource, Default)]
pub struct PathfindingQueue {
    requests: Vec<(Entity, UVec2, PathAlgorithm)>, // (实体, 目标点, 算法类型)
}

// 算法选择策略
#[derive(Debug, Clone, Copy)]
pub enum PathAlgorithm {
    DStarLite, // 长CD动物
    APF,       // 短CD动物（人工势场法）
    Hybrid,    // 中CD动物（混合策略）
}

pub fn idle_action_system(
    mut commands: Commands,
    query: Query<&BehaveCtx, With<IdleAction>>,
    mut board_query: Query<(&mut Transform, &mut AnimalActorBoard)>,
    partition: Res<SpatialPartition>,
    time: Res<Time>,
) {
    for ctx in query.iter() {
        if let Ok((mut transform, mut actor)) = board_query.get_mut(ctx.target_entity()) {
            match actor.state {
                ActorState::Idle => {
                    actor.idle_counter += 1;
                    // 保持Idle的概率：最大30%的概率，idle_counter每增加1，概率降低10%，但最小保留1%的概率。
                    if rand::random_ratio(max(99, 60 + actor.idle_counter * 10), 100) {
                        actor.state = ActorState::RandomMove;
                        actor.idle_counter = 0;
                    }
                }
                ActorState::RandomMove => {}
                _ => {
                    commands.trigger(ctx.failure());
                    return;
                }
            }
            // 运行到此处，actor处于RandomMove状态
            // 如果进入饥饿临界值，进入觅食状态
            if actor.satiety <= 50 {
                actor.state = ActorState::Seeking;
                commands.trigger(ctx.failure());
                return;
            }

            // 移动cd冷却结束，方可继续移动
            if actor.move_cd_timer.tick(time.delta()).finished() {
                actor.move_cd_timer.reset();

                actor.state = ActorState::Idle; // 移动后回归Idle状态
                let mut neighbours = get_neighbours(actor.current_pos.into());
                info!("Neighbours: {:?}", neighbours);
                loop {
                    if neighbours.is_empty() {
                        break;
                    }
                    // 随机选取一个邻近的坐标作为下次移动的目标
                    let neighbour = if neighbours.len() > 1 {
                        neighbours.remove(rand::random_range(0..(neighbours.len() - 1)))
                    } else {
                        neighbours[0]
                    };

                    if !partition.is_valid_position(&neighbour) || partition.is_obstacle(&neighbour)
                    {
                        continue;
                    }

                    info!("Move to {:?}", &neighbour);
                    actor.current_pos = neighbour.to_vec2();
                    let mut center = partition.grid_to_world(&actor.current_pos);
                    center.z = 3.0; //移动类型的entity的z轴设为3.0,以便其他类型的entity在其上方渲染。
                    transform.translation = center;
                    //TODO entity移动了，需要修改partition中的entities的数据
                    break;
                }
            }
        }
    }
}

pub fn onadd_idle_action(
    _trigger: Trigger<OnAdd, IdleAction>,
    _q: Query<&BehaveCtx, With<IdleAction>>,
) {
    info!("==IdleAction:OnAdd==");
}
