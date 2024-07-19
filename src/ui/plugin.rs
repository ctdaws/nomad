use bevy::app::{App, Plugin, Startup, Update};

use super::{
    encounter::{
        process_encounter_button_presses, setup_encounter, update_encounter, UpdateEncounter,
    },
    game_over::{check_for_game_over, setup_game_over},
    home::{
        close_home_ui, hide_open_home_ui_button, open_home_ui, setup_home_ui,
        show_open_home_ui_button, store_and_take_resources, HideOpenHomeUIButton,
        ShowOpenHomeUIButton,
    },
    resources::{setup_resources, update_resources, UpdateResources},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateResources>()
            .add_event::<UpdateEncounter>()
            .add_event::<ShowOpenHomeUIButton>()
            .add_event::<HideOpenHomeUIButton>()
            .add_systems(
                Startup,
                (
                    setup_encounter,
                    setup_resources,
                    setup_home_ui,
                    setup_game_over,
                ),
            )
            .add_systems(
                Update,
                (
                    update_resources,
                    update_encounter,
                    process_encounter_button_presses,
                    store_and_take_resources,
                    check_for_game_over,
                    open_home_ui,
                    close_home_ui,
                    show_open_home_ui_button,
                    hide_open_home_ui_button,
                ),
            );
    }
}
