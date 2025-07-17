use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, ButtonInput};

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
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

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

/// 注册镜头控制系统
pub fn register_camera_controller(app: &mut App) {
    app.add_systems(Update, camera_controller_system);
}