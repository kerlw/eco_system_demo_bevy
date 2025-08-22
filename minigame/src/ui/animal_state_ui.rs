use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AnimalStateUIPanel {
    pub parent: Option<Entity>,
}

impl AnimalStateUIPanel {
    pub fn new(parent: Entity) -> Self {
        Self {
            parent: Some(parent),
        }
    }
}

