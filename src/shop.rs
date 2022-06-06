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
        app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_shop_grid));
    }
}

#[derive(Default, Debug, WidgetProps, Clone, PartialEq)]
pub struct ItemProps {
    pub tea: Tea,
    pub handle: u16,
}

#[derive(Default, Debug, WidgetProps, Clone, PartialEq, Copy)]
pub struct ShopProps {
    pub black_tea_handle: u16,
}

fn handle_shop_click(context: &mut KayakContextRef, event: &mut Event, props: &ItemProps) {
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

#[widget]
pub fn Shop(props: ShopProps) {
    let row_styles = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        ..Style::default()
    };

    let money_binding =
        context.query_world::<Res<Binding<f32>>, _, _>(move |player| player.clone());

    context.bind(&money_binding);

    rsx! {
        <>
        <Element styles={Some(row_styles)}>
        <Item tea={Tea {
            tea_type: TeaType::BlackTea,
            cost: 100.0
        }} handle={props.black_tea_handle} />
        <Item tea={Tea {
            tea_type: TeaType::BlackTea,
            cost: 200.0
        }} handle={props.black_tea_handle} />
        <Item tea={Tea {
            tea_type: TeaType::BlackTea,
            cost: 300.0
        }} handle={props.black_tea_handle} />
    </Element>
    <Element styles={Some(row_styles)}>
    </Element>
    <Element styles={Some(row_styles)}>
    </Element>
    </>
    }
}

#[widget]
pub fn Item(props: ItemProps) {
    let button_styles = context.query_world::<Query<&Player>, _, _>(|player_query| {
        let player = player_query.single();
        let mut style = Style {
            position_type: StyleProp::Value(PositionType::ParentDirected),
            //padding: StyleProp::Value(Edge::all(Units::Pixels(100.0))),
            border_radius: StyleProp::Value(Corner::all(30.0)),
            left: StyleProp::Value(Units::Pixels(10.0)),
            top: StyleProp::Value(Units::Pixels(10.0)),
            width: StyleProp::Value(Units::Pixels(250.0)),
            height: StyleProp::Value(Units::Pixels(250.0)),
            background_color: StyleProp::Value(KayakColor::new(
                123.0 / 255.0,
                63.0 / 255.0,
                0.0,
                0.0,
            )),
            ..Style::default()
        };
        if player.money < props.tea.cost {
            style.background_color = StyleProp::Value(KayakColor::new(0.1, 0.1, 0.1, 0.0));
        }
        style
    });

    let image_styles = Style {
        position_type: StyleProp::Value(PositionType::ParentDirected),
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Pixels(200.0)),
        ..Style::default()
    };

    let text_styles = Style {
        font_size: StyleProp::Value(24.0),
        ..Style::default()
    };

    let props_clone = props.clone();
    let on_click_event = OnEvent::new(move |context, event| {
        handle_shop_click(context, event, &props_clone);
    });

    let handle = props.handle;
    let item_name = format!("{:?} {}", props.tea.tea_type, props.tea.cost);

    rsx! {
        <>
                <KayakButton on_event={Some(on_click_event)} styles={Some(button_styles)}>
                    <KayakText styles={Some(text_styles)} content={item_name} />
                    <KayakImage styles={Some(image_styles)} handle={handle} />
                </KayakButton>
        </>
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
