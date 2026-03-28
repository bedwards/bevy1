use bevy::prelude::*;

/// Plugin that provides common application lifecycle behavior shared across all games.
///
/// Currently handles graceful exit on Escape key. Future additions may include
/// pause state management, screenshot capture, and debug overlay toggling.
pub struct AppLifecyclePlugin;

impl Plugin for AppLifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_exit);
    }
}

fn handle_exit(keyboard: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
