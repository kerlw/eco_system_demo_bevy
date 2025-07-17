use bevy::input::{ButtonInput, keyboard::KeyCode};
use bevy::prelude::*;

/// 镜头控制器组件
#[derive(Component)]
pub struct CameraController {
    pub move_speed: f32,
    pub zoom_speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            move_speed: 500.0,
            zoom_speed: 1.0,
        }
    }
}

/// 镜头控制系统
pub fn camera_controller_system(
    time: ResMut<Time>,
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &CameraController)>,
) {
    for (mut transform, controller) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        // WASD移动控制
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        // 没有按键的时候可以返回了,如果下面的缩放要添加实现，这里需要修改。
        if direction == Vec3::ZERO {
            return;
        }

        // info!("direction: {:?}", direction);

        // 归一化方向向量（避免斜向移动更快）
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        // 应用移动速度和时间增量
        let movement = direction * controller.move_speed * time.delta_secs();
        transform.translation += movement;

        // 缩放控制（示例）
        if keyboard_input.pressed(KeyCode::Equal) {
            // 放大
        }
        if keyboard_input.pressed(KeyCode::Minus) {
            // 缩小
        }
    }
}

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        // 注册镜头控制系统
        app.add_systems(Update, camera_controller_system);
    }
}
