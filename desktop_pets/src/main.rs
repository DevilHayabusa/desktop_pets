use bevy::prelude::*;
use bevy::window::{Window, WindowLevel, WindowResolution};
use chrono::{Local, Timelike};

// --- Resources ---
// Caches our image handles so we only load them from disk once
#[derive(Resource)]
struct PetSprites {
    idle: Handle<Image>,
    sleep: Handle<Image>,
}

// Timer to prevent checking the OS clock 60 times a second
#[derive(Resource)]
struct CircadianTimer(Timer);

// --- Components ---
// Defines the current state of our pet
#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
enum PetStatus {
    Idle,
    Sleeping,
}

#[derive(Component)]
struct PetSpriteMarker;

#[derive(Component)]
struct AssetScaledMarker;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        // Check the clock every 5 seconds
        .insert_resource(CircadianTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Desktop Pet Engine".into(),
                transparent: true,
                decorations: false,
                window_level: WindowLevel::AlwaysOnTop,
                resizable: false,
                resolution: WindowResolution::new(120, 120),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_assets_and_spawn_pet))
        .add_systems(Update, (
            scale_pet_to_fit_system,
            drag_window_system,
            check_circadian_rhythm_system,
            update_sprite_state_system
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// System: Load assets once and spawn the entity
fn setup_assets_and_spawn_pet(mut commands: Commands, asset_server: Res<AssetServer>) {
    let idle_handle = asset_server.load("hero.png");
    let sleep_handle = asset_server.load("sleep.png");

    // Store handles globally in a resource
    commands.insert_resource(PetSprites {
        idle: idle_handle.clone(),
        sleep: sleep_handle,
    });

    // Spawn the pet with its initial state
    commands.spawn((
        Sprite::from_image(idle_handle),
        PetSpriteMarker,
        PetStatus::Idle,
    ));
}

// System: Dynamically scale the sprite to fit our 120x120 window
fn scale_pet_to_fit_system(
    mut commands: Commands,
    mut pet_query: Query<(Entity, &mut Transform, &Sprite), (With<PetSpriteMarker>, Without<AssetScaledMarker>)>,
    images: Res<Assets<Image>>,
) {
    for (entity, mut transform, sprite) in pet_query.iter_mut() {
        if let Some(image) = images.get(&sprite.image) {
            let texture_width = image.texture_descriptor.size.width as f32;
            let texture_height = image.texture_descriptor.size.height as f32;
            let max_dim = texture_width.max(texture_height);
            
            if max_dim > 0.0 {
                let scale_factor = 120.0 / max_dim;
                transform.scale = Vec3::splat(scale_factor);
                commands.entity(entity).insert(AssetScaledMarker);
            }
        }
    }
}

// System: Evaluates OS time and updates state logic
fn check_circadian_rhythm_system(
    time: Res<Time>,
    mut timer: ResMut<CircadianTimer>,
    mut query: Query<&mut PetStatus, With<PetSpriteMarker>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let hour = Local::now().hour();
        
        // Sleep between 10 PM (22) and 7 AM (7)
        let new_status = if hour >= 22 || hour < 7 {
            PetStatus::Sleeping
        } else {
            PetStatus::Idle
        };

        for mut status in query.iter_mut() {
            // Only update if the status actually changed
            if *status != new_status {
                *status = new_status;
                println!("Pet state changed to: {:?}", new_status);
            }
        }
    }
}

// System: Listens for state changes and applies visual updates
fn update_sprite_state_system(
    sprites: Res<PetSprites>,
    // The 'Changed' filter ensures this system ONLY runs the exact frame the status changes
    mut query: Query<(&PetStatus, &mut Sprite), (With<PetSpriteMarker>, Changed<PetStatus>)>,
) {
    for (status, mut sprite) in query.iter_mut() {
        match status {
            PetStatus::Idle => sprite.image = sprites.idle.clone(),
            PetStatus::Sleeping => sprite.image = sprites.sleep.clone(),
        }
    }
}

fn drag_window_system(
    mut windows: Query<&mut Window>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(mut window) = windows.single_mut() {
            window.start_drag_move();
        }
    }
}