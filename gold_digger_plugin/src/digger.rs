use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::map::{Map, MapSystemLabels, MapTile, MiningEffect, PlayerCamera, Tile};
use crate::GameState;
use bevy::prelude::*;

pub struct DiggerPlugin;

const Y_OFFSET_TO_DIGGER_BOTTOM: f32 = 10.;
const LEFT_OFFSET_TO_DIGGER_BORDER: f32 = 11.;
const RIGHT_OFFSET_TO_DIGGER_BORDER: f32 = 12.;

#[derive(SystemLabel, Eq, PartialEq, Hash, Clone, Debug)]
pub enum DiggerSystemLabels {
    MoveDigger,
    MarkMiningTarget,
}

impl Plugin for DiggerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DiggerState>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(spawn_digger.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_digger.system().label(DiggerSystemLabels::MoveDigger))
                    .with_system(
                        mark_mining_target
                            .system()
                            .label(DiggerSystemLabels::MarkMiningTarget)
                            .after(DiggerSystemLabels::MoveDigger),
                    )
                    .with_system(loose_fuel.system())
                    .with_system(update_fall_and_fly.system())
                    .with_system(dig.system().after(DiggerSystemLabels::MarkMiningTarget)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(
                    despawn_digger
                        .system()
                        .before(MapSystemLabels::DespawnMapAndCamera),
                ),
            );
    }
}

pub struct Digger;

pub struct DiggerState {
    pub waste: usize,
    pub dead: bool,
    pub money: f32,
    pub fuel: f32,
    pub fuel_max: f32,
    pub mining_strength: f32,
    pub mining_target: Option<(usize, usize)>,
    pub mining: f32,
    pub falling: bool,
    pub falling_speed: f32,
}

impl Default for DiggerState {
    fn default() -> Self {
        DiggerState {
            waste: 0,
            mining_target: None,
            dead: false,
            money: 0.,
            mining_strength: 10.,
            mining: 0.,
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
            transform: Transform::from_translation(Vec3::new(
                map.base.x,
                map.base.y + map.tile_size,
                1.,
            )),
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
    if actions.mining_down && digger_state.falling_speed == 0. {
        // this is only possible if the player is not actively pressing anything else. So the player should be standing still.
        for digger_transform in digger_query.iter_mut() {
            let slot_current_y = (digger_transform.translation.y / map.tile_size).round() as usize;
            let slot_current_x = (digger_transform.translation.x / map.tile_size).round() as usize;
            let tile_below = &map.tiles[slot_current_y - 1][slot_current_x];
            if tile_below.collides() && tile_below.mining_strength().is_some() {
                digger_state.mining_target = Some((slot_current_x, slot_current_y - 1));
                digger_state.mining += digger_state.mining_strength * time.delta_seconds();
                return;
            }
        }
    }
    let mut x = 0.;
    let mut y = 0.;
    if let Some(drive) = actions.player_movement {
        let speed = 200.;
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
            let slot_y = (digger_transform.translation.y / map.tile_size).round() as usize;
            let next_tile = &map.tiles[slot_y][slot_next_x];
            if next_tile.collides() {
                x = 0.;
                if next_tile.mining_strength().is_some() {
                    digger_state.mining_target = Some((slot_next_x, slot_y));
                    digger_state.mining += digger_state.mining_strength * time.delta_seconds();
                }
            }
        } else {
            digger_state.mining = 0.;
            digger_state.mining_target = None;
        }

        digger_transform.translation.y += y;
        digger_transform.translation.x += x;
        for mut transform in camera_query.iter_mut() {
            transform.translation = digger_transform.translation;
        }
    }
}

fn loose_fuel(mut digger_state: ResMut<DiggerState>, time: Res<Time>) {
    let fuel_rate = 0.5;
    digger_state.fuel -= fuel_rate * time.delta_seconds();
    digger_state.fuel = digger_state.fuel.clamp(0., digger_state.fuel_max);
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
    } else {
        digger_state.falling_speed = 0.;
    }
    digger_state.falling_speed = digger_state.falling_speed.clamp(-falling_rate, flying_rate);
}

fn dig(
    mut commands: Commands,
    mut digger_state: ResMut<DiggerState>,
    mut map: ResMut<Map>,
    mut tile_query: Query<(Entity, &MapTile, &mut Handle<ColorMaterial>), With<Mining>>,
    texture_assets: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if digger_state.mining_target.is_none() {
        return;
    }
    let tile =
        &map.tiles[digger_state.mining_target.unwrap().1][digger_state.mining_target.unwrap().0];
    if digger_state.mining >= tile.mining_strength().unwrap() {
        if let Some(MiningEffect::Money(value)) = tile.effect() {
            digger_state.money += value;
        } else if let Some(MiningEffect::TankUpgrade(value)) = tile.effect() {
            println!("upping fuel by {}", value);
            digger_state.fuel += value;
            digger_state.fuel_max += value;
        } else if let Some(MiningEffect::CollectedWaste) = tile.effect() {
            digger_state.waste += 1;
        }
        for (entity, map_tile, mut material) in tile_query.iter_mut() {
            if map_tile.x != digger_state.mining_target.unwrap().0
                || map_tile.y != digger_state.mining_target.unwrap().1
            {
                continue;
            }
            commands.insert(entity, Mined);
            *material = materials.add(texture_assets.texture_background.clone().into());
            map.tiles[digger_state.mining_target.unwrap().1]
                [digger_state.mining_target.unwrap().0] = Tile::Background;
            println!("reset mining status");
            digger_state.mining_target = None;
            digger_state.mining = 0.;
            break;
        }
    }
}

struct Mining;
struct Mined;

fn mark_mining_target(
    mut commands: Commands,
    mut tile_query: Query<(Entity, &MapTile, &mut Handle<ColorMaterial>), Without<Mining>>,
    mut mining_tile_query: Query<
        (Entity, &MapTile, &mut Handle<ColorMaterial>),
        (With<Mining>, Without<Mined>),
    >,
    digger_state: Res<DiggerState>,
    texture_assets: Res<TextureAssets>,
    map: Res<Map>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if digger_state.mining_target.is_none() {
        for (entity, map_tile, mut material) in mining_tile_query.iter_mut() {
            let tile = &map.tiles[map_tile.y][map_tile.x];
            *material = materials.add(texture_assets.get_tile_handle(tile).into());
            commands.remove::<Mining>(entity);
        }
    } else {
        for (entity, map_tile, mut material) in tile_query.iter_mut() {
            if map_tile.x != digger_state.mining_target.unwrap().0
                || map_tile.y != digger_state.mining_target.unwrap().1
            {
                continue;
            }
            commands.insert(entity, Mining);
            let tile = &map.tiles[map_tile.y][map_tile.x];
            if let Some(handle) = texture_assets.get_mining_tile_handle(tile) {
                *material = materials.add(handle.into());
            }
        }
    }
}

fn despawn_digger(mut commands: Commands, digger: Query<Entity, With<Digger>>) {
    for digger in digger.iter() {
        commands.despawn(digger);
    }
}
