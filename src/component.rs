use bevy::prelude::*;

#[derive(Default, Component)]
pub struct InterpTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub mix_ratio: f32,
}

impl InterpTransform {
    #[must_use]
    pub const fn with_ratio(mut self, ratio: f32) -> Self {
        self.mix_ratio = ratio;

        self
    }
}

impl From<Transform> for InterpTransform {
    fn from(value: Transform) -> Self {
        Self {
            translation: value.translation,
            rotation: value.rotation,
            scale: value.scale,
            mix_ratio: 0.5,
        }
    }
}
