use bevy::prelude::*;
use bevy_mod_interp::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InterpPlugin)
        .add_startup_system(setup)
        .add_system(update.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(1.))
        .run();
}

#[derive(Component)]
struct BoxMarker;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        InterpTransform::from(Transform::IDENTITY.with_scale(Vec3::new(50., 50., 1.))),
        BoxMarker,
    ));
}

// this function just generates the corners for the square to move to
// the only thing that is abnormal is the usage of `InterpTransform`
fn update(mut query: Query<&mut InterpTransform, With<BoxMarker>>, mut count: Local<i32>) {
    *count = (*count + 1) % 4;

    let mut transform = query.single_mut();
    transform.translation.x = (*count % 2 * 2 - 1) as f32 * 100.;
    transform.translation.y = (*count / 2 * 2 - 1) as f32 * 100.;
}
