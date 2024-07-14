use bevy::ecs::{
    event::{Event, EventWriter},
    system::ResMut,
};

use crate::{plugin::PlayerResources, ui::resources::UpdateResources};

#[derive(Event)]
pub struct AdvanceDay;

pub fn advance_day(
    mut player_resources: ResMut<PlayerResources>,
    mut update_resources_events: EventWriter<UpdateResources>,
) {
    player_resources.food -= 1;
    player_resources.water -= 1;

    update_resources_events.send(UpdateResources {
        player_food: Some(player_resources.food),
        player_water: Some(player_resources.water),
        ..Default::default()
    });
}
