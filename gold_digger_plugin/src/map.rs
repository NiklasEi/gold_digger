use crate::GameState;
use bevy::prelude::*;

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

trait Tile {
    fn asset_path(&self) -> String;
}

enum NonMinerals {
    None,
    Base,
}

impl Tile for NonMinerals {
    fn asset_path(&self) -> String {
        return match self {
            NonMinerals::Base => "base.png".to_owned(),
            NonMinerals::None => "none.png".to_owned(),
        };
    }
}

enum Mineral {
    Stone,
    Gold,
}

impl Distribution<Mineral> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mineral {
        match rng.gen_range(0..2) {
            0 => Mineral::Stone,
            _ => Mineral::Gold,
        }
    }
}

impl Tile for Mineral {
    fn asset_path(&self) -> String {
        return match self {
            Mineral::Stone => "stone.png".to_owned(),
            Mineral::Gold => "gold.png".to_owned(),
        };
    }
}

struct Dimensions {
    x: u16,
    y: u16,
}

pub struct Map {
    dimensions: Dimensions,
    pub tiles: Vec<Vec<String>>,
    pub base: Vec2,
    pub tile_size: f32,
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
            vec![NonMinerals::None]
                .iter()
                .cycle()
                .take(map.dimensions.x as usize)
                .map(|elem| elem.asset_path())
                .collect(),
        );
    }

    for _base_rows in 10..12 {
        let mut row = vec![];
        row.append(
            &mut vec![NonMinerals::None]
                .iter()
                .cycle()
                .take((map.dimensions.x as usize) / 2 - 1)
                .map(|elem| elem.asset_path())
                .collect(),
        );
        row.append(&mut vec![
            NonMinerals::Base.asset_path(),
            NonMinerals::Base.asset_path(),
        ]);
        row.append(
            &mut vec![NonMinerals::None]
                .iter()
                .cycle()
                .take((map.dimensions.x as usize) / 2 - 1)
                .map(|elem| elem.asset_path())
                .collect(),
        );
        map.tiles.push(row);
    }

    for _stone_row in 12..map.dimensions.y {
        let mut row: Vec<String> = vec![];
        for _column in 0..map.dimensions.x {
            let mineral = random::<Mineral>();
            row.push(mineral.asset_path());
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
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Rendering map");
    for row in 0..map.dimensions.y {
        for column in 0..map.dimensions.x {
            let tile = &map.tiles[row as usize][column as usize];
            println!(
                "render {} at {}/{}",
                tile,
                column as f32 * map.tile_size,
                row as f32 * map.tile_size
            );
            let handle = asset_server.load(&tile[..]);
            commands.spawn(SpriteBundle {
                material: materials.add(handle.into()),
                transform: Transform::from_translation(Vec3::new(
                    column as f32 * map.tile_size,
                    row as f32 * map.tile_size,
                    0.,
                )),
                ..Default::default()
            });
        }
    }
}
