use crate::{
    ai::Satiety,
    ui::{Percentage, ProgressBarMaterial},
};

use super::behave_tree::AnimalActorBoard;
use bevy::{math::ops::floor, prelude::*};

#[derive(Resource, Default)]
pub struct FrameCounter {
    pub counter: u32,
    pub elpased: f32,
}

impl FrameCounter {
    pub fn reset(&mut self) {
        self.counter = 0;
        self.elpased = 0.0;
    }
}

pub fn udpate_board_state_system(
    mut query: Query<(&mut AnimalActorBoard, &Children)>,
    mut f_counter: ResMut<FrameCounter>,
    mut pbar_q: Query<(&mut Satiety, &MeshMaterial2d<ProgressBarMaterial>)>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
    time: Res<Time>,
) {
    f_counter.elpased += time.delta_secs();
    f_counter.counter += 1;

    if f_counter.counter % 10 == 0 {
        for (mut board, children) in query.iter_mut() {
            board.satiety -= floor(f_counter.elpased * board.decay_faction * 100f32) as i32;
            for child in children {
                if let Ok((mut satiety, material)) = pbar_q.get_mut(*child) {
                    satiety.0 = board.satiety;
                    materials.get_mut(material.id()).map(|m| {
                        m.value_and_dimensions.x = satiety.value();
                    });
                }
            }
            // info!("satiety:{}", board.satiety);
        }

        f_counter.reset();
    }
}
