use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct DefaultEntity(pub Entity);

impl DefaultEntity {
    pub fn is_default(&self) -> bool {
        self.0 == Entity::PLACEHOLDER
    }
    pub fn set_default(&mut self) {
        self.0 = Entity::PLACEHOLDER
    }
}
impl Default for DefaultEntity {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
