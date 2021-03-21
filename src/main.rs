use bevy::prelude::{App, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use the_cleanup_plugin::GamePlugin;

fn main() {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "The Cleanup".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
