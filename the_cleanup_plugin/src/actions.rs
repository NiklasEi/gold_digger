use crate::GameState;
use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions.system()),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<f32>,
    pub flying: bool,
    pub mining_down: bool,
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_released(KeyCode::W)
        || keyboard_input.just_pressed(KeyCode::W)
        || keyboard_input.just_released(KeyCode::A)
        || keyboard_input.just_released(KeyCode::D)
        || keyboard_input.pressed(KeyCode::A)
        || keyboard_input.pressed(KeyCode::D)
    {
        actions.mining_down = false;
        let mut player_movement = actions.player_movement.unwrap_or(0.);

        if keyboard_input.just_released(KeyCode::W) {
            actions.flying = false;
        } else if keyboard_input.just_pressed(KeyCode::W) {
            actions.flying = true;
        }

        if keyboard_input.just_released(KeyCode::D) || keyboard_input.just_released(KeyCode::A) {
            if keyboard_input.pressed(KeyCode::D) {
                player_movement = 1.;
            } else if keyboard_input.pressed(KeyCode::A) {
                player_movement = -1.;
            } else {
                player_movement = 0.;
            }
        } else if keyboard_input.just_pressed(KeyCode::D) {
            player_movement = 1.;
        } else if keyboard_input.just_pressed(KeyCode::A) {
            player_movement = -1.;
        }

        actions.player_movement = Some(player_movement);
    } else {
        actions.player_movement = None;
        if keyboard_input.just_released(KeyCode::S) || keyboard_input.just_pressed(KeyCode::S) {
            actions.mining_down = keyboard_input.just_pressed(KeyCode::S);
        }
    }
}
