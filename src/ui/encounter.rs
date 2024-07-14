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
    sprite::Sprite,
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
        events::{
            set_new_location_state, MoveToLocation, ShowConnectedLocations,
            SpawnLocationConnections,
        },
        location::{
            self, ConnectedLocations, CurrentLocation, Encounter, LocationId, LocationState,
            Locations,
        },
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
    pub interactions: Vec<location::Interaction>,
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
            for encounter_option in ev.interactions.clone() {
                parent
                    .spawn((
                        encounter_option.clone(),
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
                            encounter_option.text.clone(),
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
    encounter_buttons_query: Query<(Entity, &Children), With<EncounterButtons>>,
    interaction_query: Query<
        (&Interaction, &location::Interaction),
        (Changed<Interaction>, With<Button>),
    >,
    mut encounter_ui_query: Query<&mut Visibility, With<EncounterUI>>,
    mut connected_locations_query: Query<&mut ConnectedLocations>,
    mut sprite_and_state_query: Query<(&mut Sprite, &mut LocationState)>,
    mut encounter_query: Query<&mut Encounter>,
    mut update_resources: EventWriter<UpdateResources>,
    mut spawn_location_connections_events: EventWriter<SpawnLocationConnections>,
    mut show_connected_locations_events: EventWriter<ShowConnectedLocations>,
    mut move_to_location_events: EventWriter<MoveToLocation>,
) {
    let mut button_pressed = false;

    for (_, children) in encounter_buttons_query.iter() {
        for &child in children {
            if let Ok((interaction, encounter_interaction)) = interaction_query.get(child) {
                if *interaction == Interaction::Pressed {
                    if let Some(food) = encounter_interaction.food {
                        player_resources.food += food;
                    }

                    if let Some(water) = encounter_interaction.water {
                        player_resources.water += water;
                    }

                    if let Some(wood) = encounter_interaction.wood {
                        if wood < 0 && (player_resources.wood + wood < 0) {
                            continue;
                        } else {
                            player_resources.wood += wood;
                        }
                    }

                    if let Some(location_id) = encounter_interaction.unlocks_location {
                        let mut connected_locations = connected_locations_query
                            .get_mut(locations.0[&current_location.0])
                            .unwrap();

                        connected_locations.0.push(LocationId(location_id));

                        move_to_location_events.send(MoveToLocation(current_location.0));

                        let mut connected_locations = connected_locations_query
                            .get_mut(locations.0[&location_id])
                            .unwrap();

                        connected_locations.0.push(LocationId(current_location.0));

                        let mut encounter = encounter_query
                            .get_mut(locations.0[&current_location.0])
                            .unwrap();

                        encounter
                            .interactions
                            .retain(|i| i.unlocks_location.is_none());

                        let mut encounter =
                            encounter_query.get_mut(locations.0[&location_id]).unwrap();

                        encounter
                            .interactions
                            .retain(|i| i.unlocks_location.is_none());
                    }

                    update_resources.send(UpdateResources {
                        player_food: Some(player_resources.food),
                        player_water: Some(player_resources.water),
                        player_wood: Some(player_resources.wood),
                        ..Default::default()
                    });

                    *encounter_ui_query.single_mut() = Visibility::Hidden;
                    button_pressed = true;
                }
            }
        }
    }

    if button_pressed {
        for (entity, _) in encounter_buttons_query.iter() {
            commands.entity(entity).despawn_descendants();
        }
    }
}
