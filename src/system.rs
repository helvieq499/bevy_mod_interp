use bevy::prelude::*;
use lerp::Lerp;

use crate::InterpTransform;

// TODO: add query filters like Changed
#[allow(clippy::needless_pass_by_value)] // straight up incorrect lint
pub fn update_interp(time: Res<Time>, mut query: Query<(&mut Transform, &mut InterpTransform)>) {
    let delta = time.delta_seconds();
    if delta == 0. {
        return;
    }

    for (mut transform, mut interp) in query.iter_mut() {
        // from https://youtu.be/YJB1QnEmlTs?t=472
        interp.mix_ratio = 1.0 - interp.mix_ratio.powf(delta);

        transform.translation = Vec3::new(
            transform.translation.x.lerp(interp.translation.x, interp.mix_ratio),
            transform.translation.y.lerp(interp.translation.y, interp.mix_ratio),
            transform.translation.z.lerp(interp.translation.z, interp.mix_ratio),
        );

        // TODO: interpolate rotation and scale
        transform.rotation = interp.rotation;
        transform.scale = interp.scale;
    }
}
