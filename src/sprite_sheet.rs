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
    // the handle points to the location where the asset was loaded: faster and reduces mem usage!
    let texture_handle = {
        // loader is not ECS: it's a resource that belongs to the ECS World
        let loader = world.read_resource::<Loader>(); // added by the core application

        // texture storage is not ECS: it's a resource where the loader puts Textures loaded from spritesheet, etc.
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "textures/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "textures/spritesheet.ron",        // ron file: sprites' positions
        SpriteSheetFormat(texture_handle), // handle to the loaded spritesheet png file
        (),
        &sprite_sheet_store, // storage used to store spritesheet data (extract pics from the png file)
    )
}
