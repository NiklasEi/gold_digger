use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::map::{Map, PlayerCamera};
use crate::GameState;
use bevy::prelude::*;

pub struct DiggerPlugin;

const Y_OFFSET_TO_DIGGER_BOTTOM: f32 = 10.;
const LEFT_OFFSET_TO_DIGGER_BORDER: f32 = 12.;
const RIGHT_OFFSET_TO_DIGGER_BORDER: f32 = 13.;

impl Plugin for DiggerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DiggerState>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(spawn_digger.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_digger.system())
                    .with_system(loose_fuel.system())
                    .with_system(fall.system()),
            );
    }
}

pub struct Digger;

pub struct DiggerState {
    pub money: f32,
    pub health: f32,
    pub health_max: f32,
    pub fuel: f32,
    pub fuel_max: f32,
    pub falling_speed: Option<f32>,
    pub flying_speed: Option<f32>,
}

impl Default for DiggerState {
    fn default() -> Self {
        DiggerState {
            money: 0.,
            health: 100.,
            health_max: 100.,
            fuel: 20.,
            fuel_max: 20.,
            falling_speed: None,
            flying_speed: None,
        }
    }
}

fn spawn_digger(
    mut commands: Commands,
    map: Res<Map>,
    texture_assets: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_assets.texture_digger.clone().into()),
            transform: Transform::from_translation(Vec3::new(map.base.x, map.base.y, 1.)),
            ..Default::default()
        })
        .with(Digger);
}

fn move_digger(
    time: Res<Time>,
    digger_state: ResMut<DiggerState>,
    actions: Res<Actions>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Digger>)>,
    mut player_query: Query<&mut Transform, (With<Digger>, Without<PlayerCamera>)>,
) {
    let mut x = 0.;
    let mut y = 0.;
    if let Some(drive) = actions.player_movement {
        let speed = 150.;
        x += drive * speed * time.delta_seconds();
    }
    if actions.flying {
        y += digger_state.flying_speed.unwrap_or(0.) * time.delta_seconds();
    } else {
        y -= digger_state.falling_speed.unwrap_or(0.) * time.delta_seconds();
    }
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation.y += y;
        player_transform.translation.x += x;
        for mut transform in camera_query.iter_mut() {
            transform.translation = player_transform.translation;
        }
    }
}

fn loose_fuel(mut digger_state: ResMut<DiggerState>, time: Res<Time>) {
    let fuel_rate = 1.;
    digger_state.fuel -= fuel_rate * time.delta_seconds();
}

fn fall(
    mut digger_state: ResMut<DiggerState>,
    time: Res<Time>,
    map: Res<Map>,
    digger_query: Query<&Transform, With<Digger>>,
) {
    let falling_rate = 500.;
    if let Ok(transform) = digger_query.single() {
        let digger_bottom = transform.translation.y - Y_OFFSET_TO_DIGGER_BOTTOM;
        let slot_x_left = ((transform.translation.x - LEFT_OFFSET_TO_DIGGER_BORDER) / map.tile_size)
            .round() as usize;
        let slot_x_right = ((transform.translation.x + RIGHT_OFFSET_TO_DIGGER_BORDER)
            / map.tile_size)
            .round() as usize;
        let slot_y = (digger_bottom / map.tile_size).round() as usize;

        let tile_left = &map.tiles[slot_y][slot_x_left];
        let tile_right = &map.tiles[slot_y][slot_x_right];
        let mut current_falling_speed = digger_state.falling_speed.unwrap_or(0.);
        if tile_left.collides() || tile_right.collides() {
            digger_state.falling_speed = None;
        } else {
            current_falling_speed += falling_rate * time.delta_seconds();
            current_falling_speed = current_falling_speed.clamp(0., 500.);
            digger_state.falling_speed = Some(current_falling_speed);
        }
    }
}
