mod actions;
mod audio;
mod base;
mod digger;
mod loading;
mod map;
mod menu;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::base::BasePlugin;
use crate::digger::DiggerPlugin;
use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use crate::ui::UiPlugin;
use bevy::app::AppBuilder;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::schedule::SystemSet;
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq)]
enum GameState {
    Restart,
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(DiggerPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(BasePlugin)
            .add_plugin(InternalAudioPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Restart).with_system(switch_to_game.system()),
            )
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }
}

fn switch_to_game(mut state: ResMut<State<GameState>>) {
    state.set_next(GameState::Playing).unwrap();
}
