use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::WorldExt,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    shred::World,
};

/// Reads a spritesheet png file and its related ron file in order to parse the spritesheet
/// at the appropriate positions in order to generate a handle to a well-formatted SpriteSheet
/// struct which contains the sprites at the right sizes/positions.
pub fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let asset_loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    // handle for the textures/sprites
    let texture_handle = asset_loader.load(
        "textures/spritesheet.png",
        ImageFormat::default(),
        (),
        &texture_storage,
    );

    // final handle for the textures given the sprites of the ron sprite positions
    asset_loader.load(
        "textures/spritesheet.ron",
        SpriteSheetFormat(texture_handle), // uses ron file to load sprites from spritesheet
        (),
        &sprite_sheet_storage,
    )
}
