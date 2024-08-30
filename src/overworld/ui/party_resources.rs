use bevy::{
    color::{Color, LinearRgba},
    ecs::{
        component::Component,
        event::{Event, EventReader},
        query::With,
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    text::{Text, TextSection, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        Display, FlexDirection, PositionType, Style, Val,
    },
    utils::default,
};

use crate::overworld::party_resources::PartyResources;

#[derive(Event)]
pub struct UpdateFoodUIEvent(pub i32);
#[derive(Event)]
pub struct UpdateWaterUIEvent(pub i32);
#[derive(Event)]
pub struct UpdateWoodUIEvent(pub i32);

#[derive(Component)]
pub struct PartyResourcesUI;

#[derive(Component)]
pub struct PartyFoodUI;
#[derive(Component)]
pub struct PartyWaterUI;
#[derive(Component)]
pub struct PartyWoodUI;

pub fn setup_party_resources(mut commands: Commands, party_resources: Res<PartyResources>) {
    commands
        .spawn((
            PartyResourcesUI,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(150.),
                    height: Val::Px(100.),
                    right: Val::Px(0.),
                    top: Val::Px(10.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PartyFoodUI,
                TextBundle::from_sections([
                    TextSection::new(
                        "Food: ",
                        TextStyle {
                            font_size: 30.,
                            color: Color::LinearRgba(LinearRgba::WHITE),
                            ..Default::default()
                        },
                    ),
                    TextSection::new(
                        party_resources.food.to_string(),
                        TextStyle {
                            font_size: 30.,
                            color: Color::LinearRgba(LinearRgba::WHITE),
                            ..Default::default()
                        },
                    ),
                ]),
            ));

            parent.spawn((
                PartyWaterUI,
                TextBundle::from_sections([
                    TextSection::new(
                        "Water: ",
                        TextStyle {
                            font_size: 30.,
                            color: Color::LinearRgba(LinearRgba::WHITE),
                            ..Default::default()
                        },
                    ),
                    TextSection::new(
                        party_resources.water.to_string(),
                        TextStyle {
                            font_size: 30.,
                            color: Color::LinearRgba(LinearRgba::WHITE),
                            ..Default::default()
                        },
                    ),
                ]),
            ));

            parent.spawn((
                PartyWoodUI,
                TextBundle::from_sections([
                    TextSection::new(
                        "Wood: ",
                        TextStyle {
                            font_size: 30.,
                            color: Color::LinearRgba(LinearRgba::WHITE),
                            ..Default::default()
                        },
                    ),
                    TextSection::new(
                        party_resources.wood.to_string(),
                        TextStyle {
                            font_size: 30.,
                            color: Color::LinearRgba(LinearRgba::WHITE),
                            ..Default::default()
                        },
                    ),
                ]),
            ));
        });
}

pub fn update_food(
    mut update_food_events: EventReader<UpdateFoodUIEvent>,
    mut query: Query<&mut Text, With<PartyFoodUI>>,
) {
    for ev in update_food_events.read() {
        query.single_mut().sections[1].value = ev.0.to_string()
    }
}

pub fn update_water(
    mut update_water_events: EventReader<UpdateWaterUIEvent>,
    mut query: Query<&mut Text, With<PartyWaterUI>>,
) {
    for ev in update_water_events.read() {
        query.single_mut().sections[1].value = ev.0.to_string()
    }
}

pub fn update_wood(
    mut update_wood_events: EventReader<UpdateWoodUIEvent>,
    mut query: Query<&mut Text, With<PartyWoodUI>>,
) {
    for ev in update_wood_events.read() {
        query.single_mut().sections[1].value = ev.0.to_string()
    }
}
