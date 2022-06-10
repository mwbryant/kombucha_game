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
        .continue_to_state(GameState::OurStore)
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
        .add_system_set(SystemSet::on_enter(GameState::OurStore).with_system(spawn_bottle))
        .add_system(bottle_sprite_updating)
        .add_system(exit_shop)
        .add_startup_system(spawn_player)
        .add_plugin(ShopPlugin)
        .add_plugin(BevyKayakUIPlugin)
        .register_inspectable::<Player>()
        .register_inspectable::<TeaType>()
        .register_inspectable::<Tea>()
        .run();
}

fn exit_shop(mut state: ResMut<State<GameState>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        let _ = state.pop();
    }
    if keyboard.just_pressed(KeyCode::Tab) {
        let _ = state.push(GameState::Shop);
    }
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
    let sheet_size = 512;
    let tile_size = 32;
    let rows = sheet_size / tile_size;
    let empty_bottle_index = rows + 1;

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(empty_bottle_index),
            texture_atlas: images.sprite_sheet.clone(),
            ..default()
        })
        .insert(Clickable {
            hitbox: Vec2::splat(32.0),
        })
        .insert(Bottle::default())
        .insert(Name::new("Bottle 1"));
}
fn bottle_sprite_updating(mut bottles: Query<(&mut TextureAtlasSprite, &Bottle), Changed<Bottle>>) {
    let sheet_size = 512;
    let tile_size = 32;
    let rows = sheet_size / tile_size;
    let filled_bottle_index = 0;
    let empty_bottle_index = rows + 1;

    for (mut sprite, bottle) in bottles.iter_mut() {
        if bottle.tea.is_some() {
            sprite.index = filled_bottle_index;
        } else {
            sprite.index = empty_bottle_index;
        }
    }
}

fn click_detection(
    mouse_position: Res<MousePosition>,
    clickables: Query<(Entity, &Clickable, &GlobalTransform)>,
    mut bottles: Query<&mut Bottle>,
    mouse: Res<Input<MouseButton>>,
    mut player: Query<&mut Player>,
) {
    let mut player = player.single_mut();
    if mouse.just_pressed(MouseButton::Left) {
        for (entity, clickable, transform) in clickables.iter() {
            if collide(
                transform.translation,
                clickable.hitbox,
                mouse_position.0.extend(0.0),
                Vec2::splat(1.0),
            )
            .is_some()
            {
                if let Ok(mut bottle) = bottles.get_mut(entity) {
                    if bottle.tea.is_none() {
                        if let Some(tea) = player.teas.pop() {
                            bottle.tea = Some(tea);
                        }
                    }
                }
            }
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
