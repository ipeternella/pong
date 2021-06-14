use amethyst::{
    assets::Loader,
    audio::{OggFormat, SourceHandle},
    prelude::WorldExt,
    shred::World,
};

use crate::settings::{BOUNCE_SFX, SCORE_SFX};

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

// Loads an audio asset file and returns a handle (fd) to it.
fn load_audio_file(sound_file_path: &str, asset_loader: &Loader, world: &World) -> SourceHandle {
    let storage = world.read_resource();

    asset_loader.load(sound_file_path, OggFormat, (), &storage)
}

pub fn initialize_audio(world: &mut World) {
    let sounds = {
        // immutable borrow ends in this block scope
        let asset_loader = world.read_resource::<Loader>();
        let sounds = Sounds {
            score_sfx: load_audio_file(SCORE_SFX, &asset_loader, world),
            bounce_sfx: load_audio_file(BOUNCE_SFX, &asset_loader, world),
        };

        sounds
    };

    world.insert(sounds);
}
