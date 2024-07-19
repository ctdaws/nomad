use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::{Changed, With},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, Children, DespawnRecursiveExt},
    render::{color::Color, view::Visibility},
    text::{Text, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignContent, AlignItems, BackgroundColor, Display, FlexDirection, Interaction,
        JustifyContent, PositionType, Style, Val,
    },
};

use crate::{
    locations::{
        events::{MoveToLocation, ReduceLocationEncounterLevel},
        location::{self, ConnectedLocations, CurrentLocation, LocationId, Locations},
    },
    plugin::PlayerResources,
};

use super::resources::UpdateResources;

#[derive(Component)]
pub struct EncounterUI;

#[derive(Component)]
pub struct EncounterText;

#[derive(Component)]
pub struct EncounterButtons;

#[derive(Event)]
pub struct UpdateEncounter {
    pub text: String,
    pub button: Option<location::Button>,
    pub can_ignore_encounter: bool,
}

pub fn setup_encounter(mut commands: Commands) {
    commands
        .spawn((
            EncounterUI,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(800.),
                    height: Val::Px(550.),
                    left: Val::Px(350.),
                    top: Val::Px(175.),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLUE.with_a(0.75)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                EncounterText,
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font_size: 30.,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                }
                .with_style(Style {
                    width: Val::Px(780.),
                    height: Val::Px(200.),
                    top: Val::Px(10.),
                    left: Val::Px(10.),
                    ..Default::default()
                }),
            ));

            parent.spawn((
                EncounterButtons,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(780.),
                        height: Val::Px(320.),
                        left: Val::Px(10.),
                        top: Val::Px(220.),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
        });
}

pub fn update_encounter(
    mut ev_reader: EventReader<UpdateEncounter>,
    mut query_encounter_text: Query<&mut Text, With<EncounterText>>,
    query_encounter_buttons: Query<Entity, With<EncounterButtons>>,
    mut commands: Commands,
    mut encounter_ui_query: Query<&mut Visibility, With<EncounterUI>>,
) {
    for ev in ev_reader.read() {
        let mut text = query_encounter_text.single_mut();
        text.sections[0].value = ev.text.clone();

        let encounter_buttons = query_encounter_buttons.single();

        commands.entity(encounter_buttons).with_children(|parent| {
            if let Some(button) = &ev.button {
                parent
                    .spawn((
                        button.clone(),
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(700.0),
                                height: Val::Px(65.0),
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::GRAY),
                            ..Default::default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            button.text.clone(),
                            TextStyle {
                                font_size: 30.,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        ));
                    });
            }

            if ev.can_ignore_encounter {
                parent
                    .spawn((
                        location::Button::default(),
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(700.0),
                                height: Val::Px(65.0),
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::GRAY),
                            ..Default::default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Continue on".to_string(),
                            TextStyle {
                                font_size: 30.,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        ));
                    });
            }
        });

        *encounter_ui_query.single_mut() = Visibility::Visible;
    }
}

pub fn process_encounter_button_presses(
    mut commands: Commands,
    mut player_resources: ResMut<PlayerResources>,
    locations: Res<Locations>,
    current_location: Res<CurrentLocation>,
    interaction_query: Query<
        (&Interaction, &location::Button),
        (Changed<Interaction>, With<Button>),
    >,
    encounter_buttons_query: Query<(Entity, &Children), With<EncounterButtons>>,
    mut encounter_ui_query: Query<&mut Visibility, With<EncounterUI>>,
    mut connected_locations_query: Query<&mut ConnectedLocations>,
    mut update_resources: EventWriter<UpdateResources>,
    mut move_to_location_events: EventWriter<MoveToLocation>,

    mut reduce_location_encounter_level_events: EventWriter<ReduceLocationEncounterLevel>,
) {
    let mut button_pressed = false;

    if let Ok((encounter_buttons_entity, children)) = encounter_buttons_query.get_single() {
        for &child in children {
            if let Ok((interaction, button)) = interaction_query.get(child) {
                if !matches!(*interaction, Interaction::Pressed) {
                    continue;
                }

                if *button != location::Button::default() {
                    if let Some(food) = button.food {
                        player_resources.food += food;
                    }

                    if let Some(water) = button.water {
                        player_resources.water += water;
                    }

                    if let Some(wood) = button.wood {
                        if wood < 0 && (player_resources.wood + wood < 0) {
                            continue;
                        } else {
                            player_resources.wood += wood;
                        }
                    }

                    if let Some(location_id) = button.unlocks_location {
                        let mut connected_locations = connected_locations_query
                            .get_mut(locations.0[&current_location.0])
                            .unwrap();

                        connected_locations.0.push(LocationId(location_id));

                        move_to_location_events.send(MoveToLocation(current_location.0));

                        let mut connected_locations = connected_locations_query
                            .get_mut(locations.0[&location_id])
                            .unwrap();

                        connected_locations.0.push(LocationId(current_location.0));

                        reduce_location_encounter_level_events
                            .send(ReduceLocationEncounterLevel(location_id));
                    }

                    reduce_location_encounter_level_events
                        .send(ReduceLocationEncounterLevel(current_location.0));

                    update_resources.send(UpdateResources {
                        player_food: Some(player_resources.food),
                        player_water: Some(player_resources.water),
                        player_wood: Some(player_resources.wood),
                        ..Default::default()
                    });
                }

                *encounter_ui_query.single_mut() = Visibility::Hidden;
                button_pressed = true;
            }
        }

        if button_pressed {
            commands
                .entity(encounter_buttons_entity)
                .despawn_descendants();
        }
    }
}
