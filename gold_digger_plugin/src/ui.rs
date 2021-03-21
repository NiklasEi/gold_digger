use crate::digger::DiggerState;
use crate::GameState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(init_life.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_game_state.system())
                    .with_system(retry_system.system())
                    .with_system(click_retry_button.system()),
            );
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        }
    }
}

struct RetryButton;

struct HealthText;

struct FuelText;

struct MoneyText;

fn init_life(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    digger_state: Res<DiggerState>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());
    commands.spawn(UiCameraBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!(
                                "Health: {}/{}",
                                digger_state.health.round(),
                                digger_state.health_max
                            ),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.6, 0.6, 0.6),
                                ..Default::default()
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .with(HealthText);
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(50.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!(
                                "Fuel: {}/{}",
                                digger_state.fuel.round(),
                                digger_state.fuel_max
                            ),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.6, 0.6, 0.6),
                                ..Default::default()
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .with(FuelText);
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("$ {}", digger_state.money),
                            style: TextStyle {
                                font_size: 40.0,
                                font: font.clone(),
                                color: Color::rgb(0.6, 0.6, 0.6),
                                ..Default::default()
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .with(MoneyText);
        });
}

fn update_game_state(
    digger_state: Res<DiggerState>,
    mut health_query: Query<&mut Text, (With<HealthText>, Without<MoneyText>, Without<FuelText>)>,
    mut score_query: Query<&mut Text, (With<MoneyText>, Without<HealthText>, Without<FuelText>)>,
    mut fuel_query: Query<&mut Text, (With<FuelText>, Without<HealthText>, Without<MoneyText>)>,
) {
    for mut text in health_query.iter_mut() {
        text.sections.first_mut().unwrap().value = format!(
            "Health: {}/{}",
            digger_state.health.round(),
            digger_state.health_max
        );
    }
    for mut text in score_query.iter_mut() {
        text.sections.first_mut().unwrap().value = format!("$ {}", digger_state.money);
    }
    for mut text in fuel_query.iter_mut() {
        text.sections.first_mut().unwrap().value = format!(
            "Fuel: {}/{}",
            digger_state.fuel.round(),
            digger_state.fuel_max
        );
    }
}

fn retry_system(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    digger_state: Res<DiggerState>,
    button_materials: Res<ButtonMaterials>,
) {
    if digger_state.health <= 0. || digger_state.fuel <= 0. {
        commands
            .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .with(RetryButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Restart".to_string(),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                ..Default::default()
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                });
            });
    }
}

fn click_retry_button(
    mut commands: Commands,
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<GameState>>,
    mut digger_state: ResMut<DiggerState>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
    text_query: Query<Entity, With<Text>>,
) {
    for (button, interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *digger_state = DiggerState::default();
                commands.despawn(button);
                commands.despawn(text);
                state.set_next(GameState::Restart).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}
