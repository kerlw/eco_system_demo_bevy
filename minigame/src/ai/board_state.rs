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
    mut query: Query<&mut AnimalActorBoard>,
    mut f_counter: ResMut<FrameCounter>,
    time: Res<Time>,
) {
    f_counter.elpased += time.delta_secs();
    f_counter.counter += 1;

    if f_counter.counter % 10 == 0 {
        for mut board in &mut query {
            board.satiety -= floor(f_counter.elpased * board.decay_faction * 100f32) as i32;
            // info!("satiety:{}", board.satiety);
        }

        f_counter.reset();
    }
}
