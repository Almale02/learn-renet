use bevy::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct NetworkSprite {
    /// The sprite's color tint
    pub color: Color,
    /// Flip the sprite along the `X` axis
    pub flip_x: bool,
    /// Flip the sprite along the `Y` axis
    pub flip_y: bool,
    /// An optional custom size for the sprite that will be used when rendering, instead of the size
    /// of the sprite's image
    pub custom_size: Option<Vec2>,
    /// An optional rectangle representing the region of the sprite's image to render, instead of
    /// rendering the full image. This is an easy one-off alternative to using a texture atlas.
    pub rect: Option<Rect>,
}

impl From<NetworkSprite> for Sprite {
    fn from(value: NetworkSprite) -> Self {
        Sprite {
            color: value.color,
            flip_x: value.flip_x,
            flip_y: value.flip_y,
            custom_size: value.custom_size,
            rect: value.rect,
            anchor: bevy::sprite::Anchor::Center,
        }
    }
}

impl From<Sprite> for NetworkSprite {
    fn from(val: Sprite) -> Self {
        Self {
            color: val.color,
            flip_x: val.flip_x,
            flip_y: val.flip_y,
            custom_size: val.custom_size,
            rect: val.rect,
        }
    }
}
