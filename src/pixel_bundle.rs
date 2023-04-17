use bevy::{
    prelude::{Bundle, ComputedVisibility, GlobalTransform, Handle, Image, Transform, Visibility},
    render::texture::DEFAULT_IMAGE_HANDLE,
    sprite::{ColorMaterial, Sprite, TextureAtlas, TextureAtlasSprite},
};

#[derive(Bundle, Clone)]
pub struct PixelSpriteBundle {
    pub sprite: Sprite,
    /// A handle to the material that affects the sprite texture
    pub material: Handle<ColorMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl Default for PixelSpriteBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform: Default::default(),
            material: Default::default(),
            global_transform: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}
/// A Bundle of components for drawing a single sprite from a sprite sheet (also referred
/// to as a `TextureAtlas`)
#[derive(Bundle, Clone, Default)]
pub struct PixelSpriteSheetBundle {
    /// The specific sprite from the texture atlas to be drawn, defaulting to the sprite at index 0.
    pub sprite: TextureAtlasSprite,
    /// A handle to the material that affects the texture atlas
    pub material: Handle<ColorMaterial>,
    /// A handle to the texture atlas that holds the sprite images
    pub texture_atlas: Handle<TextureAtlas>,
    /// Data pertaining to how the sprite is drawn on the screen
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}
