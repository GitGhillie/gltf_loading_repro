use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;

#[derive(Resource)]
struct SceneHandles {
    handle_a: UntypedHandle,
    handle_b: UntypedHandle,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, load_scenes)
        // Give it some time to load the scenes. Just for illustration purposes, normally you'd use a better solution
        .add_systems(Update, spawn_scenes.run_if(on_timer(Duration::new(1, 0))))
        .run();
}

fn load_scenes(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Loading");
    let handle_a = asset_server.load_untyped("A.glb").untyped();
    let handle_b = asset_server.load_untyped("B.glb").untyped();

    // Commenting this out will also prevent spawn_scenes from being able to spawn the scenes
    commands.insert_resource(SceneHandles { handle_a, handle_b })
}

// A should be spawned to the left of B.
// Roughly 25% of the time it's actually the other way around.
fn spawn_scenes(mut commands: Commands, asset_server: Res<AssetServer>, mut executed: Local<bool>) {
    if *executed {
        return;
    } else {
        *executed = true
    }

    commands.spawn((SceneBundle {
        scene: asset_server.load("A.glb"),
        transform: Transform::from_xyz(-1.0, 0.0, 0.0),
        ..default()
    },));

    commands.spawn((SceneBundle {
        scene: asset_server.load("B.glb"),
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        ..default()
    },));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 4.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 1.0, 5.0),
        ..default()
    });
}
