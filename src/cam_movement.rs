use bevy::prelude::*;
use bevy_flycam::MovementSettings;

/// bevy system for handling the camera movement speed and reacting to keypresses +, -
/// and pass the info into the camera movement settings
pub fn camera_movement_speed(
    keyboard_input: Res<Input<KeyCode>>,
    mut movement_settings: ResMut<MovementSettings>,
) {
    if keyboard_input.just_pressed(KeyCode::NumpadAdd) {
        movement_settings.speed *= 1.5;
        info!("camera speed: {}", movement_settings.speed);
    }
    if keyboard_input.just_pressed(KeyCode::NumpadSubtract) {
        movement_settings.speed /= 1.5;
        info!("camera speed: {}", movement_settings.speed);
    }
}