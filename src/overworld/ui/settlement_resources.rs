use bevy::{
    color::{Color, LinearRgba},
    ecs::{
        component::Component,
        event::{Event, EventReader},
        query::With,
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    render::view::Visibility,
    text::{Text, TextSection, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        Display, FlexDirection, PositionType, Style, Val,
    },
    utils::default,
};

use crate::{
    locations::location::{CurrentLocation, LocationId},
    settlement_resources::SettlementResources,
};

#[derive(Event)]
pub struct UpdateSettlementFoodUIEvent(pub i32);
#[derive(Event)]
pub struct UpdateSettlementWaterUIEvent(pub i32);
#[derive(Event)]
pub struct UpdateSettlementWoodUIEvent(pub i32);

#[derive(Component)]
pub struct SettlementResourcesUI;

#[derive(Component)]
pub struct SettlementFoodUI;
#[derive(Component)]
pub struct SettlementWaterUI;
#[derive(Component)]
pub struct SettlementWoodUI;

pub fn setup_settlement_resources_ui(
    mut commands: Commands,
    settlement_resources: Res<SettlementResources>,
) {
    commands
        .spawn((
            SettlementResourcesUI,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(150.),
                    height: Val::Px(100.),
                    right: Val::Px(200.),
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
                SettlementFoodUI,
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
                        settlement_resources.food.to_string(),
                        TextStyle {
                            font_size: 30.,
                            color: Color::LinearRgba(LinearRgba::WHITE),
                            ..Default::default()
                        },
                    ),
                ]),
            ));

            parent.spawn((
                SettlementWaterUI,
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
                        settlement_resources.water.to_string(),
                        TextStyle {
                            font_size: 30.,
                            color: Color::LinearRgba(LinearRgba::WHITE),
                            ..Default::default()
                        },
                    ),
                ]),
            ));

            parent.spawn((
                SettlementWoodUI,
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
                        settlement_resources.wood.to_string(),
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

pub fn update_settlement_food(
    mut update_settlement_food_events: EventReader<UpdateSettlementFoodUIEvent>,
    mut query: Query<&mut Text, With<SettlementFoodUI>>,
) {
    for ev in update_settlement_food_events.read() {
        query.single_mut().sections[1].value = ev.0.to_string()
    }
}

pub fn update_settlement_water(
    mut update_settlement_water_events: EventReader<UpdateSettlementWaterUIEvent>,
    mut query: Query<&mut Text, With<SettlementWaterUI>>,
) {
    for ev in update_settlement_water_events.read() {
        query.single_mut().sections[1].value = ev.0.to_string()
    }
}

pub fn update_settlement_wood(
    mut update_settlement_wood_events: EventReader<UpdateSettlementWoodUIEvent>,
    mut query: Query<&mut Text, With<SettlementWoodUI>>,
) {
    for ev in update_settlement_wood_events.read() {
        query.single_mut().sections[1].value = ev.0.to_string()
    }
}

pub fn update_settlement_resources_ui_visibility(
    current_location: Res<CurrentLocation>,
    mut visibility_query: Query<&mut Visibility, With<SettlementResourcesUI>>,
) {
    if current_location.0 == LocationId(0) {
        *visibility_query.single_mut() = Visibility::Visible
    } else {
        *visibility_query.single_mut() = Visibility::Hidden
    }
}
