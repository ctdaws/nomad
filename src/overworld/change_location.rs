use bevy::ecs::event::{Event, EventReader};

#[derive(Event)]
pub struct ChangeLocationEvent;

pub fn change_location(mut change_location_events: EventReader<ChangeLocationEvent>) {
    for ev in change_location_events.read() {
        println!("Change location");
    }
}
