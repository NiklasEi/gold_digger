use crate::GameState;
use bevy::prelude::*;

use crate::loading::TextureAssets;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{random, Rng};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GeneratingMap).with_system(generate_map.system()),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_camera.system())
                .with_system(render_map.system()),
        );
    }
}

pub struct PlayerCamera;

#[derive(PartialEq, Clone)]
pub enum Tile {
    None,
    Background,
    Base,
    Stone,
    Gold,
}

impl Tile {
    pub fn collides(&self) -> bool {
        match self {
            &Tile::Stone => true,
            &Tile::Gold => true,
            _ => false,
        }
    }

    pub fn mining_strength(&self) -> Option<f32> {
        match self {
            &Tile::Stone => Some(15.),
            &Tile::Gold => Some(20.),
            _ => None,
        }
    }

    pub fn value(&self) -> f32 {
        match self {
            &Tile::Gold => 10.,
            _ => 0.,
        }
    }
}

impl Distribution<Tile> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tile {
        match rng.gen_range(0..2) {
            0 => Tile::Stone,
            _ => Tile::Gold,
        }
    }
}

struct Dimensions {
    x: usize,
    y: usize,
}

pub struct Map {
    dimensions: Dimensions,
    pub tiles: Vec<Vec<Tile>>,
    pub base: Vec2,
    pub tile_size: f32,
}

pub struct MapTile {
    pub x: usize,
    pub y: usize,
}

fn generate_map(mut commands: Commands, mut state: ResMut<State<GameState>>) {
    let mut map = Map {
        dimensions: Dimensions { x: 50, y: 100 },
        tiles: vec![],
        tile_size: 32.,
        base: Vec2::new(24.5 * 32., 88. * 32.),
    };

    for _sky_rows in 0..10 {
        map.tiles.push(
            vec![Tile::None]
                .iter()
                .cycle()
                .take(map.dimensions.x as usize)
                .cloned()
                .collect(),
        );
    }

    for _base_rows in 10..12 {
        let mut row = vec![];
        row.append(
            &mut vec![Tile::None]
                .iter()
                .cycle()
                .take((map.dimensions.x as usize) / 2 - 1)
                .cloned()
                .collect(),
        );
        row.append(&mut vec![Tile::Base, Tile::Base]);
        row.append(
            &mut vec![Tile::None]
                .iter()
                .cycle()
                .take((map.dimensions.x as usize) / 2 - 1)
                .cloned()
                .collect(),
        );
        map.tiles.push(row);
    }

    for _stone_row in 12..map.dimensions.y {
        let mut row: Vec<Tile> = vec![];
        for _column in 0..map.dimensions.x {
            row.push(random::<Tile>().clone());
        }
        map.tiles.push(row);
    }
    map.tiles.reverse();

    commands.insert_resource(map);
    state.set_next(GameState::Playing).unwrap();
}

fn spawn_camera(mut commands: Commands, map: Res<Map>) {
    commands
        .spawn(OrthographicCameraBundle {
            transform: Transform::from_xyz(map.base.x, map.base.y, 999.),
            ..OrthographicCameraBundle::new_2d()
        })
        .with(PlayerCamera);
}

fn render_map(
    mut commands: Commands,
    map: Res<Map>,
    texture_assets: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for row in 0..map.dimensions.y {
        for column in 0..map.dimensions.x {
            let tile = &map.tiles[row as usize][column as usize];

            let handle = texture_assets.get_tile_handle(tile);
            commands
                .spawn(SpriteBundle {
                    material: materials.add(handle.into()),
                    transform: Transform::from_translation(Vec3::new(
                        column as f32 * map.tile_size,
                        row as f32 * map.tile_size,
                        0.,
                    )),
                    ..Default::default()
                })
                .with(MapTile { x: column, y: row });
        }
    }
}
