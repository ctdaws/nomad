use bevy::ecs::event::{Event, EventReader};

use crate::locations::setup::LocationId;

#[derive(Event)]
pub struct ChangeLocationEvent(pub LocationId);

pub fn change_location(mut change_location_events: EventReader<ChangeLocationEvent>) {
    for ev in change_location_events.read() {
        println!("Change location");
    }
}
