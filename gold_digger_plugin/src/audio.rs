use crate::actions::Actions;
use crate::digger::{DiggerState, FuelUpgrade, WasteCollected};
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AudioChannels {
            digging: AudioChannel::new("digging".to_owned()),
            flying: AudioChannel::new("flying".to_owned()),
        })
        .add_plugin(AudioPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(play_flying_and_digging_sounds.system())
                .with_system(collect_waste.system())
                .with_system(collect_fuel.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(stop_audio.system()));
    }
}

struct AudioChannels {
    flying: AudioChannel,
    digging: AudioChannel,
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.set_volume_in_channel(0.3, &channels.flying);
    audio.set_volume_in_channel(0.3, &channels.digging);
    audio.play_looped_in_channel(audio_assets.flying.clone(), &channels.flying);
    audio.play_looped_in_channel(audio_assets.digging.clone(), &channels.digging);
    audio.pause_channel(&channels.flying);
    audio.pause_channel(&channels.digging);
}

fn stop_audio(audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.stop_channel(&channels.flying);
    audio.stop_channel(&channels.digging);
}

fn play_flying_and_digging_sounds(
    digger_state: Res<DiggerState>,
    actions: Res<Actions>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
) {
    if actions.flying {
        audio.resume_channel(&channels.flying);
    } else {
        audio.pause_channel(&channels.flying)
    }

    if digger_state.mining_target.is_some() {
        audio.resume_channel(&channels.digging);
    } else {
        audio.pause_channel(&channels.digging);
    }
}

fn collect_waste(
    mut events: EventReader<WasteCollected>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for _event in events.iter() {
        audio.play(audio_assets.waste.clone());
    }
}

fn collect_fuel(
    mut events: EventReader<FuelUpgrade>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for _event in events.iter() {
        audio.play(audio_assets.fuel.clone());
    }
}
