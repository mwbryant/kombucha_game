#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{prelude::*, sprite::collide_aabb::collide, window::PresentMode};
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorParams, WorldInspectorPlugin};

mod prelude;
use kayak_ui::bevy::BevyKayakUIPlugin;
use prelude::*;
mod mouse;
use mouse::{mouse_position, MousePosition};
use shop::ShopPlugin;
mod assets;
mod shop;
mod ui;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    let mut app = App::new();
    AssetLoader::new(GameState::Splash)
        .continue_to_state(GameState::Gameplay)
        .with_collection::<ImageAssets>()
        .build(&mut app);
    app.add_state(GameState::Splash)
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Kombucha Game".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .insert_resource(MousePosition(Vec2::ZERO))
        .add_system(toggle_inspector)
        .add_system(mouse_position)
        .add_system(click_detection)
        .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_bottle))
        .add_startup_system(spawn_player)
        .add_plugin(ShopPlugin)
        .add_plugin(BevyKayakUIPlugin)
        .register_inspectable::<Player>()
        .register_inspectable::<TeaType>()
        .register_inspectable::<Tea>()
        .run();
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player {
            money: 500.0,
            teas: Vec::new(),
        })
        .insert(Name::new("Player"));
}

fn spawn_bottle(mut commands: Commands, images: Res<ImageAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: images.sprite_sheet.clone(),
            ..default()
        })
        .insert(Clickable {
            hitbox: Vec2::splat(32.0),
        })
        .insert(Name::new("Bottle 1"));
}

fn click_detection(
    mouse_position: Res<MousePosition>,
    clickables: Query<(&Clickable, &GlobalTransform)>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for (clickable, transform) in clickables.iter() {
            if collide(
                transform.translation,
                clickable.hitbox,
                mouse_position.0.extend(0.0),
                Vec2::splat(1.0),
            )
            .is_some()
            {}
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 1.0 / 3.0;

    commands.spawn_bundle(camera);
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled
    }
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
}
