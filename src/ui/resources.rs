// use bevy::{
//     ecs::{
//         component::Component,
//         event::{Event, EventReader, EventWriter},
//         query::{With, Without},
//         system::{Commands, Query, Res},
//     },
//     hierarchy::BuildChildren,
//     render::color::Color,
//     text::{Text, TextSection, TextStyle},
//     ui::{
//         node_bundles::{NodeBundle, TextBundle},
//         Display, FlexDirection, PositionType, Style, Val,
//     },
// };

// use crate::plugin::{PlayerResources, SettlementResources};

// #[derive(Component)]
// pub struct Resources;

// #[derive(Component)]
// pub struct PlayerResourcesMarker;

// #[derive(Component)]
// pub struct SettlementResourcesMarker;

// #[derive(Component)]
// pub struct PlayerFoodText;

// #[derive(Component)]
// pub struct PlayerWaterText;

// #[derive(Component)]
// pub struct PlayerWoodText;

// #[derive(Component)]
// pub struct SettlementFoodText;

// #[derive(Component)]
// pub struct SettlementWaterText;

// #[derive(Component)]
// pub struct SettlementWoodText;

// #[derive(Event, Default)]
// pub struct UpdateResources {
//     pub settlement_food: Option<i32>,
//     pub settlement_water: Option<i32>,
//     pub settlement_wood: Option<i32>,
//     pub player_food: Option<i32>,
//     pub player_water: Option<i32>,
//     pub player_wood: Option<i32>,
// }

// pub fn setup_resources(
//     mut commands: Commands,
//     player_resources: Res<PlayerResources>,
//     settlement_resources: Res<SettlementResources>,
//     mut resources_events: EventWriter<UpdateResources>,
// ) {
//     commands
//         .spawn((
//             Resources,
//             NodeBundle {
//                 style: Style {
//                     position_type: PositionType::Absolute,
//                     width: Val::Px(350.),
//                     height: Val::Px(100.),
//                     left: Val::Px(1100.),
//                     top: Val::Px(10.),
//                     display: Display::Flex,
//                     flex_direction: FlexDirection::Column,
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//         ))
//         .with_children(|parent| {
//             parent
//                 .spawn((
//                     PlayerResourcesMarker,
//                     NodeBundle {
//                         style: Style {
//                             position_type: PositionType::Absolute,
//                             width: Val::Px(150.),
//                             height: Val::Px(100.),
//                             left: Val::Px(200.),
//                             display: Display::Flex,
//                             flex_direction: FlexDirection::Column,
//                             ..Default::default()
//                         },
//                         ..Default::default()
//                     },
//                 ))
//                 .with_children(|parent| {
//                     parent.spawn((
//                         TextBundle::from_sections([
//                             TextSection::new(
//                                 "Food: ",
//                                 TextStyle {
//                                     font_size: 30.,
//                                     color: Color::WHITE,
//                                     ..Default::default()
//                                 },
//                             ),
//                             TextSection::new(
//                                 "",
//                                 TextStyle {
//                                     font_size: 30.,
//                                     color: Color::WHITE,
//                                     ..Default::default()
//                                 },
//                             ),
//                         ]),
//                         PlayerFoodText,
//                     ));

//                     parent.spawn((
//                         TextBundle::from_sections([
//                             TextSection::new(
//                                 "Water: ",
//                                 TextStyle {
//                                     font_size: 30.,
//                                     color: Color::WHITE,
//                                     ..Default::default()
//                                 },
//                             ),
//                             TextSection::new(
//                                 "",
//                                 TextStyle {
//                                     font_size: 30.,
//                                     color: Color::WHITE,
//                                     ..Default::default()
//                                 },
//                             ),
//                         ]),
//                         PlayerWaterText,
//                     ));

//                     parent.spawn((
//                         TextBundle::from_sections([
//                             TextSection::new(
//                                 "Wood: ",
//                                 TextStyle {
//                                     font_size: 30.,
//                                     color: Color::WHITE,
//                                     ..Default::default()
//                                 },
//                             ),
//                             TextSection::new(
//                                 "",
//                                 TextStyle {
//                                     font_size: 30.,
//                                     color: Color::WHITE,
//                                     ..Default::default()
//                                 },
//                             ),
//                         ]),
//                         PlayerWoodText,
//                     ));
//                 });
//         });

//     resources_events.send(UpdateResources {
//         player_food: Some(player_resources.food),
//         player_water: Some(player_resources.water),
//         player_wood: Some(player_resources.wood),
//         settlement_food: Some(settlement_resources.food),
//         settlement_water: Some(settlement_resources.water),
//         settlement_wood: Some(settlement_resources.wood),
//     });
// }

// pub fn update_resources(
//     mut evs: EventReader<UpdateResources>,
//     mut query_player_food: Query<
//         &mut Text,
//         (
//             With<PlayerFoodText>,
//             Without<PlayerWaterText>,
//             Without<PlayerWoodText>,
//             Without<SettlementFoodText>,
//             Without<SettlementWaterText>,
//             Without<SettlementWoodText>,
//         ),
//     >,
//     mut query_player_water: Query<
//         &mut Text,
//         (
//             With<PlayerWaterText>,
//             Without<PlayerFoodText>,
//             Without<PlayerWoodText>,
//             Without<SettlementFoodText>,
//             Without<SettlementWaterText>,
//             Without<SettlementWoodText>,
//         ),
//     >,
//     mut query_player_wood: Query<
//         &mut Text,
//         (
//             With<PlayerWoodText>,
//             Without<PlayerWaterText>,
//             Without<PlayerFoodText>,
//             Without<SettlementFoodText>,
//             Without<SettlementWaterText>,
//             Without<SettlementWoodText>,
//         ),
//     >,
//     mut query_settlement_food: Query<
//         &mut Text,
//         (
//             With<SettlementFoodText>,
//             Without<SettlementWaterText>,
//             Without<SettlementWoodText>,
//             Without<PlayerFoodText>,
//             Without<PlayerWaterText>,
//             Without<PlayerWoodText>,
//         ),
//     >,
//     mut query_settlement_water: Query<
//         &mut Text,
//         (
//             With<SettlementWaterText>,
//             Without<SettlementFoodText>,
//             Without<SettlementWoodText>,
//             Without<PlayerFoodText>,
//             Without<PlayerWaterText>,
//             Without<PlayerWoodText>,
//         ),
//     >,
//     mut query_settlement_wood: Query<
//         &mut Text,
//         (
//             With<SettlementWoodText>,
//             Without<SettlementWaterText>,
//             Without<SettlementFoodText>,
//             Without<PlayerFoodText>,
//             Without<PlayerWaterText>,
//             Without<PlayerWoodText>,
//         ),
//     >,
// ) {
//     for ev in evs.read() {
//         if let Some(food) = ev.player_food {
//             query_player_food.single_mut().sections[1].value = food.to_string();
//         }

//         if let Some(water) = ev.player_water {
//             query_player_water.single_mut().sections[1].value = water.to_string();
//         }

//         if let Some(wood) = ev.player_wood {
//             query_player_wood.single_mut().sections[1].value = wood.to_string();
//         }

//         if let Some(food) = ev.settlement_food {
//             query_settlement_food.single_mut().sections[1].value = food.to_string();
//         }

//         if let Some(water) = ev.settlement_water {
//             query_settlement_water.single_mut().sections[1].value = water.to_string();
//         }

//         if let Some(wood) = ev.settlement_wood {
//             query_settlement_wood.single_mut().sections[1].value = wood.to_string();
//         }
//     }
// }
