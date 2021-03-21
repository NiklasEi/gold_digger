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
                    .with_system(update_fall_and_fly.system()),
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
    pub falling: bool,
    pub falling_speed: f32,
}

impl Default for DiggerState {
    fn default() -> Self {
        DiggerState {
            money: 0.,
            health: 100.,
            health_max: 100.,
            fuel: 20.,
            fuel_max: 20.,
            falling: false,
            falling_speed: 0.,
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
    mut digger_state: ResMut<DiggerState>,
    actions: Res<Actions>,
    map: Res<Map>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Digger>)>,
    mut digger_query: Query<&mut Transform, (With<Digger>, Without<PlayerCamera>)>,
) {
    let mut x = 0.;
    let mut y = 0.;
    if let Some(drive) = actions.player_movement {
        let speed = 150.;
        x += drive * speed * time.delta_seconds();
    }
    y += digger_state.falling_speed * time.delta_seconds();
    for mut digger_transform in digger_query.iter_mut() {
        let slot_current_y = ((digger_transform.translation.y
            + if y > 0. {
                Y_OFFSET_TO_DIGGER_BOTTOM
            } else {
                -Y_OFFSET_TO_DIGGER_BOTTOM
            })
            / map.tile_size)
            .round() as usize;

        let new_border_translation_y = digger_transform.translation.y
            + y
            + if y > 0. {
                Y_OFFSET_TO_DIGGER_BOTTOM
            } else {
                -Y_OFFSET_TO_DIGGER_BOTTOM
            };
        let slot_x_left = ((digger_transform.translation.x - LEFT_OFFSET_TO_DIGGER_BORDER)
            / map.tile_size)
            .round() as usize;
        let slot_x_right = ((digger_transform.translation.x + RIGHT_OFFSET_TO_DIGGER_BORDER)
            / map.tile_size)
            .round() as usize;
        let slot_next_y = (new_border_translation_y / map.tile_size).round() as usize;

        if slot_current_y != slot_next_y {
            let next_tile_left = &map.tiles[slot_next_y][slot_x_left];
            let next_tile_right = &map.tiles[slot_next_y][slot_x_right];
            if next_tile_left.collides() || next_tile_right.collides() {
                digger_state.falling_speed = 0.;
                y = if y > 0. {
                    y + (map.tile_size / 2.)
                        - (new_border_translation_y % map.tile_size).round()
                        - 0.5
                } else {
                    y + (map.tile_size / 2.) - (new_border_translation_y % map.tile_size).round()
                        + 0.5
                }
            }
        }

        let slot_next_x = ((x + if x > 0. {
            digger_transform.translation.x + RIGHT_OFFSET_TO_DIGGER_BORDER
        } else {
            digger_transform.translation.x - LEFT_OFFSET_TO_DIGGER_BORDER
        }) / map.tile_size)
            .round() as usize;

        if slot_next_x != if x > 0. { slot_x_right } else { slot_x_left } {
            let next_tile = &map.tiles[slot_next_y][slot_next_x];
            if next_tile.collides() {
                x = 0.;
            }
        }

        digger_transform.translation.y += y;
        digger_transform.translation.x += x;
        for mut transform in camera_query.iter_mut() {
            transform.translation = digger_transform.translation;
        }
    }
}

fn loose_fuel(mut digger_state: ResMut<DiggerState>, time: Res<Time>) {
    let fuel_rate = 1.;
    digger_state.fuel -= fuel_rate * time.delta_seconds();
}

fn update_fall_and_fly(
    mut digger_state: ResMut<DiggerState>,
    time: Res<Time>,
    actions: Res<Actions>,
    map: Res<Map>,
    digger_query: Query<&Transform, With<Digger>>,
) {
    let falling_rate = 500.;
    let flying_rate = 300.;
    for digger_transform in digger_query.iter() {
        let new_border_translation_y =
            digger_transform.translation.y - Y_OFFSET_TO_DIGGER_BOTTOM - 1.;
        let slot_x_left = ((digger_transform.translation.x - LEFT_OFFSET_TO_DIGGER_BORDER)
            / map.tile_size)
            .round() as usize;
        let slot_x_right = ((digger_transform.translation.x + RIGHT_OFFSET_TO_DIGGER_BORDER)
            / map.tile_size)
            .round() as usize;
        let slot_next_y = (new_border_translation_y / map.tile_size).round() as usize;
        let next_tile_left = &map.tiles[slot_next_y][slot_x_left];
        let next_tile_right = &map.tiles[slot_next_y][slot_x_right];
        digger_state.falling = !next_tile_left.collides() && !next_tile_right.collides();
    }
    if actions.flying {
        digger_state.falling_speed += flying_rate * time.delta_seconds();
    } else if digger_state.falling {
        digger_state.falling_speed -= falling_rate * time.delta_seconds();
    }
    digger_state.falling_speed = digger_state.falling_speed.clamp(-falling_rate, flying_rate);
}
