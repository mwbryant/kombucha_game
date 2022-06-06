pub use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use bevy_inspector_egui::Inspectable;
use serde::Deserialize;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Gameplay,
}

#[derive(Component, Inspectable, Default, Clone, PartialEq)]
//TODO make a resource
pub struct Player {
    pub money: f32,
    pub teas: Vec<Tea>,
}

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "sprite_sheet.png")]
    #[asset(texture_atlas(
        tile_size_x = 32.,
        tile_size_y = 32.,
        columns = 16,
        rows = 16,
        padding_x = 2.,
        padding_y = 2.
    ))]
    pub sprite_sheet: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct Clickable {
    pub hitbox: Vec2,
}

#[derive(Debug, Clone, PartialEq, Copy, Inspectable)]
pub enum TeaType {
    BlackTea,
}

impl Default for TeaType {
    fn default() -> Self {
        TeaType::BlackTea
    }
}

#[derive(Component, Default, Debug, Clone, PartialEq, Copy, Inspectable)]
pub struct Tea {
    pub tea_type: TeaType,
    pub cost: f32,
}

#[derive(Default, Clone, Copy, Debug, Reflect, Deserialize)]
pub struct MyRect {
    pub pos: Vec2,
    pub size: Vec2,
}
