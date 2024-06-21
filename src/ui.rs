use bevy::{
    ecs::{
        component::Component,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res},
    },
    render::color::Color,
    text::{Text, TextSection, TextStyle},
    ui::{node_bundles::TextBundle, PositionType, Style, Val},
};

use crate::PlayerResources;

#[derive(Component)]
pub struct EncounterText;

#[derive(Component)]
pub struct FoodText;

#[derive(Component)]
pub struct WaterText;

#[derive(Component)]
pub struct WoodText;

#[derive(Event)]
pub struct UpdateEncounterText(pub String);
#[derive(Event)]
pub struct UpdateFoodValue(pub i32);
#[derive(Event)]
pub struct UpdateWaterValue(pub i32);
#[derive(Event)]
pub struct UpdateWoodValue(pub i32);

pub fn setup_ui(
    mut commands: Commands,
    player_resources: Res<PlayerResources>,
    mut food_value_events: EventWriter<UpdateFoodValue>,
    mut water_value_events: EventWriter<UpdateWaterValue>,
    mut wood_value_events: EventWriter<UpdateWoodValue>,
) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 30.,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.),
            left: Val::Px(10.),
            ..Default::default()
        }),
        EncounterText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Food: ",
                TextStyle {
                    font_size: 30.,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: 30.,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.),
            left: Val::Px(1300.),
            ..Default::default()
        }),
        FoodText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Water: ",
                TextStyle {
                    font_size: 30.,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: 30.,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(40.),
            left: Val::Px(1300.),
            ..Default::default()
        }),
        WaterText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Wood: ",
                TextStyle {
                    font_size: 30.,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: 30.,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(70.),
            left: Val::Px(1300.),
            ..Default::default()
        }),
        WoodText,
    ));

    food_value_events.send(UpdateFoodValue(player_resources.food));
    water_value_events.send(UpdateWaterValue(player_resources.water));
    wood_value_events.send(UpdateWoodValue(player_resources.wood));
}

pub fn update_encounter_text(
    mut ev_reader: EventReader<UpdateEncounterText>,
    mut query: Query<&mut Text, With<EncounterText>>,
) {
    for ev in ev_reader.read() {
        let mut text = query.single_mut();
        text.sections[0].value = ev.0.clone();
    }
}

pub fn update_food_value(
    mut ev_reader: EventReader<UpdateFoodValue>,
    mut query: Query<&mut Text, With<FoodText>>,
) {
    for ev in ev_reader.read() {
        let mut text = query.single_mut();
        text.sections[1].value = ev.0.to_string();
    }
}

pub fn update_water_value(
    mut ev_reader: EventReader<UpdateWaterValue>,
    mut query: Query<&mut Text, With<WaterText>>,
) {
    for ev in ev_reader.read() {
        let mut text = query.single_mut();
        text.sections[1].value = ev.0.to_string();
    }
}

pub fn update_wood_value(
    mut ev_reader: EventReader<UpdateWoodValue>,
    mut query: Query<&mut Text, With<WoodText>>,
) {
    for ev in ev_reader.read() {
        let mut text = query.single_mut();
        text.sections[1].value = ev.0.to_string();
    }
}
