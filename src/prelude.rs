use std::fmt;

pub use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use bevy_inspector_egui::Inspectable;
use serde::Deserialize;

pub use crate::ui::widgets::*;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum GameState {
    Splash,
    Shop,
    OurStore,
}

#[derive(Component, Inspectable, Default, Clone, PartialEq)]
pub struct Player {
    pub money: f32,
    pub teas: Vec<Tea>,
}

#[derive(Inspectable, Default, Clone, PartialEq, Debug, Copy)]
pub struct Scoby {
    pub health: f32,
    pub potency: f32,
}

#[derive(Component, Inspectable, Default, Clone, PartialEq)]
pub struct Bottle {
    pub tea: Option<Tea>,
    pub scoby: Option<Scoby>,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemType::Tea(_tea) => write!(f, "Tea"),
            ItemType::Scoby(_scoby) => write!(f, "Scoby"),
        }
    }
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

#[derive(Debug, Clone, PartialEq, Inspectable)]
pub enum ItemType {
    Tea(Tea),
    Scoby(Scoby),
}
impl Default for ItemType {
    fn default() -> Self {
        ItemType::Tea(Tea::default())
    }
}

impl Default for TeaType {
    fn default() -> Self {
        TeaType::BlackTea
    }
}

#[derive(Component, Default, Debug, Clone, PartialEq, Copy, Inspectable)]
pub struct Tea {
    pub tea_type: TeaType,
}

#[derive(Default, Clone, Copy, Debug, Reflect, Deserialize)]
pub struct MyRect {
    pub pos: Vec2,
    pub size: Vec2,
}
