use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::Entity,
    prelude::{Builder, WorldExt},
    shred::World,
    ui::{Anchor, FontAsset, LineMode, TtfFormat, UiText, UiTransform},
};

use crate::settings::SQUARE_FONT_PATH;

/// ScoreText holds the UI text that is print on the game screen.
pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

/// Uses an asset loader to fetch font assets and return a font handle to the loaded asset.
fn load_font_handle(font_location: &str, world: &mut World) -> Handle<FontAsset> {
    let asset_loader = world.read_resource::<Loader>();
    let storage = world.read_resource::<AssetStorage<FontAsset>>();

    asset_loader.load(font_location, TtfFormat, (), &storage)
}

/// Initializes the game Score board with the players scores.
pub fn intialize_scoreboard(world: &mut World) {
    // font asset handle
    let font_handle = load_font_handle(SQUARE_FONT_PATH, world);

    // transforms to position the text
    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -50.0,
        -50.0,
        1.0,
        200.0,
        50.0,
    );

    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        50.0,
        -50.0,
        1.0,
        200.0,
        50.0,
    );

    // score entities creation
    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font_handle.clone(),
            "0".to_string(),  // string to render
            [1., 1., 1., 1.], // font color
            50.,              // font size
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font_handle,
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    // inserts ScoreText as a resource into the world to be fetched later
    // ScoreText will hold references to the scores entities (transforms, etc.)
    world.insert(ScoreText { p1_score, p2_score });
}
