use bevy::prelude::*;
use bevy::window::{Window, WindowLevel, WindowResolution};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Desktop Pet Engine".into(),
                transparent: true,
                decorations: false,
                window_level: WindowLevel::AlwaysOnTop,
                // Bloqueamos el OS snapping
                resizable: false,
                // Tamaño fijo para click-through
                resolution: WindowResolution::new(120, 120),
                ..default()
            }),
            ..default()
        }))
        // Registramos nuestros sistemas
        .add_systems(Startup, (setup_camera, spawn_pet_system))
        .add_systems(Update, (scale_pet_to_fit_system, drag_window_system))
        .run();
}

// Marker components for ECS query isolation
#[derive(Component)]
struct PetSprite;

// Marker to indicate scaling has been applied, prevents infinite scaling loops
#[derive(Component)]
struct AssetScaled;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// System A (Setup): Spawn the entity, load texture, add marker
fn spawn_pet_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the image (asynchronously)
    let texture_handle = asset_server.load("hero.png");

    commands.spawn((
        Sprite::from_image(texture_handle),
        PetSprite, // Add marker component
    ));
}

// System B (Update): Wait for asset load, calculate scale, and apply
// Runs once per asset when loaded.
fn scale_pet_to_fit_system(
    mut commands: Commands,
    // Query: Any entity with PetSprite, Transform, and Sprite, but NOT YET AssetScaled
    mut pet_query: Query<(Entity, &mut Transform, &Sprite), (With<PetSprite>, Without<AssetScaled>)>,
    images: Res<Assets<Image>>, // Access loaded images
) {
    for (entity, mut transform, sprite) in pet_query.iter_mut() {
        // Try to get the image data from the assets handle in the sprite
        if let Some(image) = images.get(&sprite.image) {
            // Asset is loaded, we can now check its dimensions
            let texture_width = image.texture_descriptor.size.width as f32;
            let texture_height = image.texture_descriptor.size.height as f32;
            
            // Calculate the maximum dimension to fit inside the 120x120 window
            let max_dim = texture_width.max(texture_height);
            
            if max_dim > 0.0 {
                // Determine the scale factor: (Desired size / Original Size)
                let scale_factor = 120.0 / max_dim;
                
                // Apply the scale transformation, keeping aspect ratio
                transform.scale = Vec3::splat(scale_factor);
                
                // Important: Mark this entity as scaled so this system ignores it next frame
                commands.entity(entity).insert(AssetScaled);
            }
        }
    }
}

// Existing dragging logic remains unchanged
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