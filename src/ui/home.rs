use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        event::EventWriter,
        query::{Changed, With, Without},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::BuildChildren,
    render::{color::Color, view::Visibility},
    text::TextStyle,
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        AlignContent, AlignItems, BackgroundColor, Interaction, JustifyContent, PositionType,
        Style, Val,
    },
};

use crate::{
    locations::location::CurrentLocation,
    plugin::{PlayerResources, SettlementResources},
};

use super::resources::{SettlementResourcesMarker, UpdateResources};

#[derive(Component)]
pub struct HomeUI;

#[derive(Component)]
pub struct StoreFoodButton;

#[derive(Component)]
pub struct TakeFoodButton;

#[derive(Component)]
pub struct StoreWaterButton;

#[derive(Component)]
pub struct TakeWaterButton;

#[derive(Component)]
pub struct StoreWoodButton;

#[derive(Component)]
pub struct TakeWoodButton;

#[derive(Bundle)]
pub struct HomeUIBundle {
    pub marker: HomeUI,
    pub node: NodeBundle,
}

pub fn setup_home_ui(mut commands: Commands) {
    commands
        .spawn(HomeUIBundle {
            marker: HomeUI,
            node: NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(470.),
                    height: Val::Px(140.),
                    left: Val::Px(500.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .with_children(|parent| {
            parent
                .spawn((
                    StoreFoodButton,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
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
                        "Store 1 Food",
                        TextStyle {
                            font_size: 30.,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    ));
                });

            parent
                .spawn((
                    TakeFoodButton,
                    ButtonBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(75.),
                            width: Val::Px(150.0),
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
                        "Take 1 Food",
                        TextStyle {
                            font_size: 30.,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    ));
                });

            parent
                .spawn((
                    StoreWaterButton,
                    ButtonBundle {
                        style: Style {
                            left: Val::Px(10.),
                            width: Val::Px(150.0),
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
                        "Store 1 Water",
                        TextStyle {
                            font_size: 30.,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    ));
                });

            parent
                .spawn((
                    TakeWaterButton,
                    ButtonBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(75.),
                            left: Val::Px(160.),
                            width: Val::Px(150.0),
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
                        "Take 1 Water",
                        TextStyle {
                            font_size: 30.,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    ));
                });

            parent
                .spawn((
                    StoreWoodButton,
                    ButtonBundle {
                        style: Style {
                            left: Val::Px(20.),
                            width: Val::Px(150.0),
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
                        "Store 1 Wood",
                        TextStyle {
                            font_size: 30.,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    ));
                });

            parent
                .spawn((
                    TakeWoodButton,
                    ButtonBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(75.),
                            left: Val::Px(320.),
                            width: Val::Px(150.0),
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
                        "Take 1 Wood",
                        TextStyle {
                            font_size: 30.,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    ));
                });
        });
}

pub fn set_home_ui_visibility(
    current_location: Res<CurrentLocation>,
    mut query: Query<&mut Visibility, (With<HomeUI>, Without<SettlementResourcesMarker>)>,
    mut query_settlement_resources: Query<
        &mut Visibility,
        (With<SettlementResourcesMarker>, Without<HomeUI>),
    >,
) {
    if current_location.0 == 0 {
        *query.single_mut() = Visibility::Visible;
        *query_settlement_resources.single_mut() = Visibility::Visible;
    } else {
        *query.single_mut() = Visibility::Hidden;
        *query_settlement_resources.single_mut() = Visibility::Hidden;
    }
}

pub fn store_and_take_resources(
    store_food_button_query: Query<&Interaction, (Changed<Interaction>, With<StoreFoodButton>)>,
    take_food_button_query: Query<&Interaction, (Changed<Interaction>, With<TakeFoodButton>)>,
    store_water_button_query: Query<&Interaction, (Changed<Interaction>, With<StoreWaterButton>)>,
    take_water_button_query: Query<&Interaction, (Changed<Interaction>, With<TakeWaterButton>)>,
    store_wood_button_query: Query<&Interaction, (Changed<Interaction>, With<StoreWoodButton>)>,
    take_wood_button_query: Query<&Interaction, (Changed<Interaction>, With<TakeWoodButton>)>,
    mut settlement_resources: ResMut<SettlementResources>,
    mut player_resources: ResMut<PlayerResources>,
    mut resources_events: EventWriter<UpdateResources>,
) {
    let mut resources_updated = false;

    for interaction in &store_food_button_query {
        if matches!(*interaction, Interaction::Pressed) {
            if player_resources.food > 0 {
                settlement_resources.food += 1;
                player_resources.food -= 1;

                resources_updated = true;
            }
        }
    }

    for interaction in &take_food_button_query {
        if matches!(*interaction, Interaction::Pressed) {
            if settlement_resources.food > 0 {
                settlement_resources.food -= 1;
                player_resources.food += 1;

                resources_updated = true;
            }
        }
    }

    for interaction in &store_water_button_query {
        if matches!(*interaction, Interaction::Pressed) {
            if player_resources.water > 0 {
                settlement_resources.water += 1;
                player_resources.water -= 1;

                resources_updated = true;
            }
        }
    }

    for interaction in &take_water_button_query {
        if matches!(*interaction, Interaction::Pressed) {
            if settlement_resources.water > 0 {
                settlement_resources.water -= 1;
                player_resources.water += 1;

                resources_updated = true;
            }
        }
    }

    for interaction in &store_wood_button_query {
        if matches!(*interaction, Interaction::Pressed) {
            if player_resources.wood > 0 {
                settlement_resources.wood += 1;
                player_resources.wood -= 1;

                resources_updated = true;
            }
        }
    }

    for interaction in &take_wood_button_query {
        if matches!(*interaction, Interaction::Pressed) {
            if settlement_resources.wood > 0 {
                settlement_resources.wood -= 1;
                player_resources.wood += 1;

                resources_updated = true;
            }
        }
    }

    if resources_updated {
        resources_events.send(UpdateResources {
            player_food: Some(player_resources.food),
            player_water: Some(player_resources.water),
            player_wood: Some(player_resources.wood),
            settlement_food: Some(settlement_resources.food),
            settlement_water: Some(settlement_resources.water),
            settlement_wood: Some(settlement_resources.wood),
        });
    }
}
