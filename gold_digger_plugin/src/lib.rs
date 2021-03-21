mod actions;
mod base;
mod digger;
mod loading;
mod map;
mod menu;

use crate::actions::ActionsPlugin;
use crate::base::BasePlugin;
use crate::digger::DiggerPlugin;
use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use bevy::app::AppBuilder;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::Plugin;

#[derive(Clone, Eq, PartialEq)]
enum GameState {
    Loading,
    Playing,
    GeneratingMap,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(DiggerPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(BasePlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }
}
