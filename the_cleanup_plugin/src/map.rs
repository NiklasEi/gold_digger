use crate::GameState;
use bevy::prelude::*;

use crate::loading::TextureAssets;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{random, thread_rng, Rng};

pub struct MapPlugin;

#[derive(SystemLabel, Eq, PartialEq, Hash, Clone, Debug)]
pub enum MapSystemLabels {
    DespawnMapAndCamera,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(generate_map.exclusive_system())
                .with_system(spawn_camera.system())
                .with_system(render_map.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(
                remove_map
                    .system()
                    .label(MapSystemLabels::DespawnMapAndCamera),
            ),
        );
    }
}

pub struct PlayerCamera;

#[derive(PartialEq, Clone)]
pub enum Tile {
    Background,
    Border,
    TankUpgrade,
    Base,
    Stone,
    Gold,
    Waste,
    Diamond,
    Silver,
}

pub enum MiningEffect {
    Money(f32),
    TankUpgrade(f32),
    CollectedWaste,
}

impl Tile {
    pub fn collides(&self) -> bool {
        match self {
            &Tile::Stone => true,
            &Tile::Silver => true,
            &Tile::Gold => true,
            &Tile::Diamond => true,
            &Tile::Border => true,
            &Tile::TankUpgrade => true,
            &Tile::Waste => true,
            _ => false,
        }
    }

    pub fn mining_strength(&self) -> Option<f32> {
        match self {
            &Tile::Stone => Some(10.),
            &Tile::TankUpgrade => Some(5.),
            &Tile::Waste => Some(5.),
            &Tile::Silver => Some(20.),
            &Tile::Gold => Some(30.),
            &Tile::Diamond => Some(50.),
            _ => None,
        }
    }

    pub fn effect(&self) -> Option<MiningEffect> {
        match self {
            &Tile::Silver => Some(MiningEffect::Money(5.)),
            &Tile::Gold => Some(MiningEffect::Money(20.)),
            &Tile::Diamond => Some(MiningEffect::Money(50.)),
            &Tile::TankUpgrade => Some(MiningEffect::TankUpgrade(5.)),
            &Tile::Waste => Some(MiningEffect::CollectedWaste),
            _ => None,
        }
    }
}

impl Distribution<Tile> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tile {
        match rng.gen_range(0..100) {
            0..=4 => Tile::Silver,
            5..=6 => Tile::Gold,
            7 => Tile::Diamond,
            _ => Tile::Stone,
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

fn generate_map(mut commands: Commands) {
    let mut map = Map {
        dimensions: Dimensions { x: 50, y: 100 },
        tiles: vec![],
        tile_size: 32.,
        base: Vec2::new(24.5 * 32., 88. * 32.),
    };
    let mut rng = thread_rng();

    map.tiles.push(
        vec![Tile::Border]
            .iter()
            .cycle()
            .take(map.dimensions.x as usize)
            .cloned()
            .collect(),
    );
    for _sky_rows in 1..10 {
        let mut row = vec![];
        row.push(Tile::Border);
        for _column in 1..map.dimensions.x - 1 {
            row.push(Tile::Background);
        }
        row.push(Tile::Border);
        map.tiles.push(row);
    }

    for _base_rows in 10..12 {
        let mut row = vec![];
        row.push(Tile::Border);
        for _column in 1..(map.dimensions.x as usize) / 2 - 1 {
            row.push(Tile::Background);
        }
        row.append(&mut vec![Tile::Base, Tile::Base]);
        for _column in 1..(map.dimensions.x as usize) / 2 - 1 {
            row.push(Tile::Background);
        }
        row.push(Tile::Border);
        map.tiles.push(row);
    }

    for _stone_row in 12..map.dimensions.y - 1 {
        let mut row: Vec<Tile> = vec![];
        row.push(Tile::Border);
        for _column in 1..map.dimensions.x - 1 {
            row.push(random::<Tile>().clone());
        }
        row.push(Tile::Border);
        map.tiles.push(row);
    }
    map.tiles.push(
        vec![Tile::Border]
            .iter()
            .cycle()
            .take(map.dimensions.x as usize)
            .cloned()
            .collect(),
    );
    map.tiles.reverse();

    // distribute 6 tank extensions
    for _depth in 0..5 {
        let x: usize = rng.gen_range(1..map.dimensions.x - 1);
        let y: usize = rng.gen_range(1..map.dimensions.y - 13);

        map.tiles[y][x] = Tile::TankUpgrade;
    }
    let x: usize = rng.gen_range(1..map.dimensions.x - 1);
    map.tiles[map.dimensions.y - 15][x] = Tile::TankUpgrade;

    // distribute 10 waste
    for _depth in 0..10 {
        let x: usize = rng.gen_range(1..map.dimensions.x - 1);
        let y: usize = rng.gen_range(1..map.dimensions.y - 13);

        map.tiles[y][x] = Tile::Waste;
    }

    commands.insert_resource(map);
}

fn spawn_camera(mut commands: Commands, map: Res<Map>) {
    commands
        .spawn_bundle(OrthographicCameraBundle {
            transform: Transform::from_xyz(map.base.x, map.base.y, 999.),
            ..OrthographicCameraBundle::new_2d()
        })
        .insert(PlayerCamera);
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
                .spawn_bundle(SpriteBundle {
                    material: materials.add(handle.into()),
                    transform: Transform::from_translation(Vec3::new(
                        column as f32 * map.tile_size,
                        row as f32 * map.tile_size,
                        0.,
                    )),
                    ..Default::default()
                })
                .insert(MapTile { x: column, y: row });
        }
    }
}

fn remove_map(
    mut commands: Commands,
    map_query: Query<Entity, With<MapTile>>,
    _player_camera: Query<Entity, With<PlayerCamera>>,
) {
    for entity in map_query.iter() {
        commands.entity(entity).despawn();
    }
}
