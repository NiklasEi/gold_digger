mod paths;

use crate::loading::paths::PATHS;
use crate::map::Tile;
use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(start_loading.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_state.system()))
        .add_system_set(
            SystemSet::on_exit(GameState::Loading).with_system(clean_up_loading.system()),
        );
    }
}

struct LoadingIndicator;

pub struct LoadingState {
    textures: Vec<HandleUntyped>,
    fonts: Vec<HandleUntyped>,
    audio: Vec<HandleUntyped>,
}

pub struct FontAssets {
    pub fira_sans: Handle<Font>,
}

pub struct AudioAssets {
    pub digging: Handle<AudioSource>,
    pub flying: Handle<AudioSource>,
    pub waste: Handle<AudioSource>,
    pub fuel: Handle<AudioSource>,
}

pub struct TextureAssets {
    pub texture_digger: Handle<Texture>,
    pub texture_background: Handle<Texture>,
    pub texture_border: Handle<Texture>,
    pub texture_tank_upgrade: Handle<Texture>,
    pub texture_stone: Handle<Texture>,
    pub texture_stone_mining: Handle<Texture>,
    pub texture_silver: Handle<Texture>,
    pub texture_silver_mining: Handle<Texture>,
    pub texture_gold: Handle<Texture>,
    pub texture_gold_mining: Handle<Texture>,
    pub texture_diamond: Handle<Texture>,
    pub texture_diamond_mining: Handle<Texture>,
    pub texture_base: Handle<Texture>,
    pub texture_waste: Handle<Texture>,
}

impl TextureAssets {
    pub fn get_tile_handle(&self, mineral: &Tile) -> Handle<Texture> {
        match mineral {
            &Tile::Stone => self.texture_stone.clone(),
            &Tile::Gold => self.texture_gold.clone(),
            &Tile::Diamond => self.texture_diamond.clone(),
            &Tile::Silver => self.texture_silver.clone(),
            &Tile::Background => self.texture_background.clone(),
            &Tile::Border => self.texture_border.clone(),
            &Tile::TankUpgrade => self.texture_tank_upgrade.clone(),
            &Tile::Base => self.texture_base.clone(),
            &Tile::Waste => self.texture_waste.clone(),
        }
    }
    pub fn get_mining_tile_handle(&self, mineral: &Tile) -> Option<Handle<Texture>> {
        match mineral {
            &Tile::Stone => Some(self.texture_stone_mining.clone()),
            &Tile::Silver => Some(self.texture_silver_mining.clone()),
            &Tile::Gold => Some(self.texture_gold_mining.clone()),
            &Tile::Diamond => Some(self.texture_diamond_mining.clone()),
            _ => None,
        }
    }
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut fonts: Vec<HandleUntyped> = vec![];
    fonts.push(asset_server.load_untyped(PATHS.fira_sans));

    let mut audio: Vec<HandleUntyped> = vec![];
    audio.push(asset_server.load_untyped(PATHS.audio_digging));
    audio.push(asset_server.load_untyped(PATHS.audio_flying));
    audio.push(asset_server.load_untyped(PATHS.audio_fuel));
    audio.push(asset_server.load_untyped(PATHS.audio_waste));

    let mut textures: Vec<HandleUntyped> = vec![];
    textures.push(asset_server.load_untyped(PATHS.texture_digger));
    textures.push(asset_server.load_untyped(PATHS.texture_background));
    textures.push(asset_server.load_untyped(PATHS.texture_tank_upgrade));
    textures.push(asset_server.load_untyped(PATHS.texture_border));
    textures.push(asset_server.load_untyped(PATHS.texture_stone));
    textures.push(asset_server.load_untyped(PATHS.texture_stone_mining));
    textures.push(asset_server.load_untyped(PATHS.texture_silver));
    textures.push(asset_server.load_untyped(PATHS.texture_silver_mining));
    textures.push(asset_server.load_untyped(PATHS.texture_gold));
    textures.push(asset_server.load_untyped(PATHS.texture_gold_mining));
    textures.push(asset_server.load_untyped(PATHS.texture_diamond));
    textures.push(asset_server.load_untyped(PATHS.texture_diamond_mining));
    textures.push(asset_server.load_untyped(PATHS.texture_base));
    textures.push(asset_server.load_untyped(PATHS.texture_waste));

    commands.insert_resource(LoadingState {
        textures,
        fonts,
        audio,
    });
}

fn check_state(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.fonts.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.textures.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.audio.iter().map(|handle| handle.id))
    {
        return;
    }

    commands.insert_resource(FontAssets {
        fira_sans: asset_server.get_handle(PATHS.fira_sans),
    });

    commands.insert_resource(AudioAssets {
        digging: asset_server.get_handle(PATHS.audio_digging),
        flying: asset_server.get_handle(PATHS.audio_flying),
        waste: asset_server.get_handle(PATHS.audio_waste),
        fuel: asset_server.get_handle(PATHS.audio_fuel),
    });

    commands.insert_resource(TextureAssets {
        texture_base: asset_server.get_handle(PATHS.texture_base),
        texture_digger: asset_server.get_handle(PATHS.texture_digger),
        texture_background: asset_server.get_handle(PATHS.texture_background),
        texture_tank_upgrade: asset_server.get_handle(PATHS.texture_tank_upgrade),
        texture_border: asset_server.get_handle(PATHS.texture_border),
        texture_stone: asset_server.get_handle(PATHS.texture_stone),
        texture_stone_mining: asset_server.get_handle(PATHS.texture_stone_mining),
        texture_silver: asset_server.get_handle(PATHS.texture_silver),
        texture_silver_mining: asset_server.get_handle(PATHS.texture_silver_mining),
        texture_gold: asset_server.get_handle(PATHS.texture_gold),
        texture_gold_mining: asset_server.get_handle(PATHS.texture_gold_mining),
        texture_diamond: asset_server.get_handle(PATHS.texture_diamond),
        texture_diamond_mining: asset_server.get_handle(PATHS.texture_diamond_mining),
        texture_waste: asset_server.get_handle(PATHS.texture_waste),
    });

    state.set_next(GameState::Menu).unwrap();
}

fn clean_up_loading(mut commands: Commands, text_query: Query<Entity, With<LoadingIndicator>>) {
    for remove in text_query.iter() {
        commands.despawn_recursive(remove);
    }
}
