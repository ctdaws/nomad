use bevy::app::{App, Plugin, Startup, Update};

use super::{
    encounter::{setup_encounter, update_encounter_text, UpdateEncounter},
    resources::{setup_resources, update_resources, UpdateResources},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateResources>()
            .add_event::<UpdateEncounter>()
            .add_systems(Startup, (setup_encounter, setup_resources))
            .add_systems(Update, (update_resources, update_encounter_text));
    }
}
