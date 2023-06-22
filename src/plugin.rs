use bevy::prelude::*;

pub struct InterpPlugin;

impl Plugin for InterpPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(crate::system::update_interp.in_base_set(CoreSet::PostUpdate));
    }
}
