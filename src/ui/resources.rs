use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        event::{Event, EventReader, EventWriter},
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::{Text, TextSection, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        Display, FlexDirection, PositionType, Style, Val,
    },
};

use crate::PlayerResources;

#[derive(Component)]
pub struct Resources;

#[derive(Component)]
pub struct FoodText;

#[derive(Component)]
pub struct WaterText;

#[derive(Component)]
pub struct WoodText;

#[derive(Bundle)]
pub struct ResourcesBundle {
    pub marker: Resources,
    pub node: NodeBundle,
}

#[derive(Event)]
pub struct UpdateResources {
    pub food: Option<i32>,
    pub water: Option<i32>,
    pub wood: Option<i32>,
}

pub fn setup_resources(
    mut commands: Commands,
    player_resources: Res<PlayerResources>,
    mut resources_events: EventWriter<UpdateResources>,
) {
    let resources = commands
        .spawn(ResourcesBundle {
            marker: Resources,
            node: NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(150.),
                    height: Val::Px(100.),
                    right: Val::Px(10.),
                    top: Val::Px(10.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .id();

    let food = commands
        .spawn((
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
            ]),
            FoodText,
        ))
        .id();

    let water = commands
        .spawn((
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
            ]),
            WaterText,
        ))
        .id();

    let wood = commands
        .spawn((
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
            ]),
            WoodText,
        ))
        .id();

    commands
        .entity(resources)
        .push_children(&[food, water, wood]);

    resources_events.send(UpdateResources {
        food: Some(player_resources.food),
        water: Some(player_resources.water),
        wood: Some(player_resources.wood),
    });
}

pub fn update_resources(
    mut evs: EventReader<UpdateResources>,
    mut query_food: Query<&mut Text, (With<FoodText>, Without<WaterText>, Without<WoodText>)>,
    mut query_water: Query<&mut Text, (With<WaterText>, Without<FoodText>, Without<WoodText>)>,
    mut query_wood: Query<&mut Text, (With<WoodText>, Without<WaterText>, Without<FoodText>)>,
) {
    for ev in evs.read() {
        if let Some(food) = ev.food {
            query_food.single_mut().sections[1].value = food.to_string();
        }

        if let Some(water) = ev.water {
            query_water.single_mut().sections[1].value = water.to_string();
        }

        if let Some(wood) = ev.wood {
            query_wood.single_mut().sections[1].value = wood.to_string();
        }
    }
}
