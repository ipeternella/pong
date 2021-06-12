use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::Entity,
    prelude::{Builder, WorldExt},
    shred::World,
    ui::{Anchor, FontAsset, LineMode, TtfFormat, UiText, UiTransform},
};

/// ScoreBoard holds the score state of the game.
#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: f32,
    pub score_right: f32,
}

/// ScoreText holds the UI text that is print on the game screen.
pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

/// Initializes the game Score board with the players scores.
pub fn intialize_scoreboard(world: &mut World) {
    let font_handle = {
        // fetch from world the assets loader
        let asset_loader = world.read_resource::<Loader>();
        let storage = world.read_resource::<AssetStorage<FontAsset>>();

        // load the font asset and put into the asset storage
        asset_loader.load("fonts/square.ttf", TtfFormat, (), &storage)
    };

    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -50.,
        -50.,
        1.,
        200.,
        50.,
    );

    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        50.,
        -50.,
        1.,
        200.,
        50.,
    );

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

    world.insert(ScoreText { p1_score, p2_score });
}
