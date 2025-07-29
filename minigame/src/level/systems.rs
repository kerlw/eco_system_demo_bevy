use bevy::prelude::*;
use crate::core::components::{EntityType, Health};
use crate::level::config::EntityConfig;

#[derive(Bundle)]
struct EntityBundle {
    entity_type: EntityType,
    transform: Transform,
    health: Health,
}

pub fn spawn_entity(
    commands: &mut Commands,
    config: &EntityConfig,
) -> Entity {
    let transform = Transform::from_xyz(
        config.x as f32,
        config.y as f32,
        0.0
    );
    
    commands.spawn(EntityBundle {
        entity_type: config.entity_type.clone(),
        transform,
        health: Health { value: config.health },
    }).id()
}