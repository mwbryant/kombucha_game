use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::prelude::*;

pub fn convert_to_image(
    sprite_desc: MyRect,
    original_image: Handle<Image>,
    assets: &mut ResMut<Assets<Image>>,
) -> Handle<Image> {
    //TODO convert if mismatch
    let original_image = assets.get(original_image).unwrap();
    assert!(original_image.texture_descriptor.format == TextureFormat::Rgba8UnormSrgb);

    let mut data = Vec::default();
    //Every pixel is 4 entries in image.data
    let mut starting_index =
        (sprite_desc.pos.x + original_image.size().x * sprite_desc.pos.y) as usize;
    for _y in 0..sprite_desc.size.y as usize {
        for x in 0..sprite_desc.size.x as usize {
            let index = starting_index + x;
            //Copy 1 pixel at index
            data.push(original_image.data[index * 4]);
            data.push(original_image.data[index * 4 + 1]);
            data.push(original_image.data[index * 4 + 2]);
            data.push(original_image.data[index * 4 + 3]);
        }
        starting_index += original_image.size().y as usize;
    }

    let size = Extent3d {
        width: sprite_desc.size.x as u32,
        height: sprite_desc.size.y as u32,
        depth_or_array_layers: 1,
    };
    let image = Image::new(
        size,
        TextureDimension::D2,
        data,
        //FIXME
        TextureFormat::Rgba8UnormSrgb,
    );
    assets.add(image)
}
