use bevy::prelude::{App, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use gold_digger_plugin::GamePlugin;

fn main() {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Gold digger".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
