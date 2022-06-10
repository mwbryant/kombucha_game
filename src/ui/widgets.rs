use crate::prelude::*;
use crate::shop::handle_shop_click;

use kayak_ui::core::styles::{Corner, Edge, LayoutType};
use kayak_ui::core::{rsx, widget, Binding, OnEvent, WidgetProps};
use kayak_ui::core::{
    styles::{PositionType, Style, StyleProp, Units},
    Color as KayakColor,
};
use kayak_ui::widgets::{Button as KayakButton, Element, Image as KayakImage, Text as KayakText};

use crate::prelude::Tea;

#[derive(Default, Debug, WidgetProps, Clone, PartialEq)]
pub struct ItemProps {
    pub item: ItemType,
    pub cost: f32,
    pub handle: u16,
}

#[derive(Default, Debug, WidgetProps, Clone, PartialEq, Copy)]
pub struct ShopProps {
    pub black_tea_handle: u16,
}

#[widget]
pub fn Shop(props: ShopProps) {
    let parent_style = Style {
        layout_type: StyleProp::Value(LayoutType::Column),
        row_between: StyleProp::Value(Units::Pixels(1.0)),
        background_color: StyleProp::Value(KayakColor::new(123.0 / 255.0, 63.0 / 255.0, 1.0, 0.0)),
        width: StyleProp::Value(Units::Pixels(250.0)),
        height: StyleProp::Value(Units::Pixels((250.0 + 10.0) * 3.0)),
        padding: StyleProp::Value(Edge::all(Units::Pixels(0.0))),
        offset: StyleProp::Value(Edge::all(Units::Pixels(0.0))),
        ..Style::default()
    };
    let element_styles = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        left: StyleProp::Value(Units::Pixels(10.0)),
        top: StyleProp::Value(Units::Pixels(10.0)),
        padding: StyleProp::Value(Edge::all(Units::Pixels(0.0))),
        offset: StyleProp::Value(Edge::all(Units::Pixels(0.0))),
        ..Style::default()
    };

    let money_binding =
        context.query_world::<Res<Binding<f32>>, _, _>(move |player| player.clone());

    context.bind(&money_binding);
    info!("Redraw");

    rsx! {
        <Element styles={Some(parent_style)}>
            <Element styles={Some(element_styles)}>
                <Item item={ItemType::Tea(Tea {
                    tea_type: TeaType::BlackTea,
                })} handle={props.black_tea_handle}
                    cost={100.0} />
                <Item item={ItemType::Tea(Tea {
                    tea_type: TeaType::BlackTea,
                })} handle={props.black_tea_handle}
                    cost={200.0} />
            </Element>
            <Element styles={Some(element_styles)}>
                <Item item={ItemType::Scoby(Scoby {
                    health: 100.0,
                    potency: 100.0,
                })} handle={props.black_tea_handle}
                    cost={100.0} />
            </Element>
            <Element styles={Some(element_styles)}>
                <Item item={ItemType::Scoby(Scoby {
                    health: 100.0,
                    potency: 100.0,
                })} handle={props.black_tea_handle}
                    cost={100.0} />
            </Element>
        </Element>
    }
}

#[widget]
pub fn Item(props: ItemProps) {
    let button_styles = context.query_world::<Query<&Player>, _, _>(|player_query| {
        let player = player_query.single();
        let mut style = Style {
            position_type: StyleProp::Value(PositionType::ParentDirected),
            padding: StyleProp::Value(Edge::all(Units::Pixels(0.0))),
            border_radius: StyleProp::Value(Corner::all(30.0)),
            offset: StyleProp::Value(Edge::all(Units::Pixels(0.0))),
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
        if player.money < props.cost {
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
    let item_name = format!("{} {}", props.item, props.cost);

    rsx! {
        <>
                <KayakButton on_event={Some(on_click_event)} styles={Some(button_styles)}>
                    <KayakText styles={Some(text_styles)} content={item_name} />
                    <KayakImage styles={Some(image_styles)} handle={handle} />
                </KayakButton>
        </>
    }
}
