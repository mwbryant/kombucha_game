use crate::assets::convert_to_image;
use crate::prelude::*;
use kayak_ui::bevy::{BevyContext, FontMapping, ImageManager, UICameraBundle};
use kayak_ui::core::styles::{Corner, LayoutType};
use kayak_ui::core::{
    bind, rsx, widget, Binding, Event, EventType, KayakContextRef, MutableBound, OnEvent,
    WidgetProps,
};
use kayak_ui::core::{
    render,
    styles::{PositionType, Style, StyleProp, Units},
    Color as KayakColor, Index,
};
use kayak_ui::widgets::{
    App as KayakApp, Button as KayakButton, Element, Image as KayakImage, Text as KayakText,
};

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_shop_grid))
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay).with_system(update_money_binding),
            );
    }
}
fn update_money_binding(money: ResMut<Binding<f32>>, player: Query<&Player>) {
    let player = player.single();
    money.set(player.money);
}

pub fn handle_shop_click(context: &mut KayakContextRef, event: &mut Event, props: &ItemProps) {
    let money_binding =
        context.query_world::<ResMut<Binding<f32>>, _, _>(move |player| player.clone());

    if let EventType::Click(..) = event.event_type {
        context.query_world::<Query<&mut Player>, _, _>(|mut player_query| {
            let mut player = player_query.single_mut();
            if player.money >= props.tea.cost {
                player.money -= props.tea.cost;
                player.teas.push(props.tea);
            }
            money_binding.set(player.money);
        });
    }
}

fn spawn_shop_grid(
    mut commands: Commands,
    mut image_manager: ResMut<ImageManager>,
    image_asset: Res<ImageAssets>,
    mut image_assets: ResMut<Assets<Image>>,
    texture_assets: ResMut<Assets<TextureAtlas>>,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    //TODO set to player money
    commands.insert_resource(bind(999999.0_f32));

    let image = texture_assets
        .get(image_asset.sprite_sheet.clone())
        .unwrap()
        .texture
        .clone();

    let rect = texture_assets
        .get(image_asset.sprite_sheet.clone())
        .unwrap()
        .textures[1];

    let handle = convert_to_image(
        MyRect {
            pos: rect.min,
            size: rect.max - rect.min,
        },
        image,
        &mut image_assets,
    );
    let black_tea_handle = image_manager.get(&handle);

    let context = BevyContext::new(|context| {
        render! {
            <KayakApp>
            <Shop black_tea_handle={black_tea_handle}/>
            </KayakApp>
        }
    });

    commands.insert_resource(context);
}
