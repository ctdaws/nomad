// use bevy::{
//     ecs::{
//         component::Component,
//         event::{Event, EventReader, EventWriter},
//         query::{Changed, With, Without},
//         system::{Commands, Query, Res, ResMut},
//     },
//     hierarchy::BuildChildren,
//     render::{color::Color, view::Visibility},
//     text::{JustifyText, TextSection, TextStyle},
//     ui::{
//         node_bundles::{ButtonBundle, NodeBundle, TextBundle},
//         AlignItems, BackgroundColor, Display, FlexDirection, Interaction, JustifyContent,
//         PositionType, Style, Val,
//     },
// };

// use crate::{
//     locations::location::CurrentLocation,
//     plugin::{PlayerResources, SettlementResources},
// };

// use super::resources::{
//     SettlementFoodText, SettlementWaterText, SettlementWoodText, UpdateResources,
// };

// #[derive(Event)]
// pub struct ShowOpenHomeUIButton;

// #[derive(Event)]
// pub struct HideOpenHomeUIButton;

// #[derive(Component)]
// pub struct OpenHomeUIButton;

// #[derive(Component)]
// pub struct CloseHomeUIButton;

// #[derive(Component)]
// pub struct HomeUI;

// #[derive(Component)]
// pub struct StoreFoodButton;

// #[derive(Component)]
// pub struct TakeFoodButton;

// #[derive(Component)]
// pub struct StoreWaterButton;

// #[derive(Component)]
// pub struct TakeWaterButton;

// #[derive(Component)]
// pub struct StoreWoodButton;

// #[derive(Component)]
// pub struct TakeWoodButton;

// pub fn setup_home_ui(mut commands: Commands) {
//     commands
//         .spawn((
//             OpenHomeUIButton,
//             ButtonBundle {
//                 style: Style {
//                     top: Val::Px(10.),
//                     left: Val::Px(1060.),
//                     width: Val::Px(175.),
//                     height: Val::Px(75.),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     ..Default::default()
//                 },
//                 background_color: BackgroundColor(Color::GRAY),
//                 ..Default::default()
//             },
//         ))
//         .with_children(|parent| {
//             parent.spawn(
//                 TextBundle::from_section(
//                     "Manage Settlement",
//                     TextStyle {
//                         font_size: 30.,
//                         color: Color::WHITE,
//                         ..Default::default()
//                     },
//                 )
//                 .with_text_justify(JustifyText::Center),
//             );
//         });

//     commands
//         .spawn((
//             HomeUI,
//             NodeBundle {
//                 style: Style {
//                     position_type: PositionType::Absolute,
//                     width: Val::Px(800.),
//                     height: Val::Px(550.),
//                     left: Val::Px(350.),
//                     top: Val::Px(175.),
//                     ..Default::default()
//                 },
//                 background_color: BackgroundColor(Color::BLUE.with_a(0.75)),
//                 visibility: Visibility::Hidden,
//                 ..Default::default()
//             },
//         ))
//         .with_children(|parent| {
//             parent
//                 .spawn((
//                     CloseHomeUIButton,
//                     ButtonBundle {
//                         style: Style {
//                             position_type: PositionType::Absolute,
//                             width: Val::Px(70.),
//                             height: Val::Px(70.),
//                             right: Val::Px(10.),
//                             top: Val::Px(10.),
//                             justify_content: JustifyContent::Center,
//                             align_items: AlignItems::Center,
//                             ..Default::default()
//                         },
//                         background_color: BackgroundColor(Color::BLACK),
//                         ..Default::default()
//                     },
//                 ))
//                 .with_children(|parent| {
//                     parent.spawn(TextBundle::from_section(
//                         "X",
//                         TextStyle {
//                             font_size: 60.,
//                             color: Color::WHITE,
//                             ..Default::default()
//                         },
//                     ));
//                 });

//             parent
//                 .spawn(NodeBundle {
//                     style: Style {
//                         position_type: PositionType::Absolute,
//                         width: Val::Px(650.),
//                         height: Val::Px(160.),
//                         left: Val::Px(10.),
//                         top: Val::Px(10.),
//                         ..Default::default()
//                     },
//                     // background_color: BackgroundColor(Color::ORANGE),
//                     ..Default::default()
//                 })
//                 .with_children(|parent| {
//                     parent.spawn(
//                         TextBundle::from_section(
//                             "Settlement Resources",
//                             TextStyle {
//                                 font_size: 30.,
//                                 color: Color::WHITE,
//                                 ..Default::default()
//                             },
//                         )
//                         .with_style(Style {
//                             position_type: PositionType::Absolute,
//                             width: Val::Px(300.),
//                             height: Val::Px(30.),
//                             left: Val::Px(250.),
//                             top: Val::Px(40.),
//                             ..Default::default()
//                         }), // .with_background_color(Color::RED),
//                     );

//                     parent
//                         .spawn(NodeBundle {
//                             style: Style {
//                                 position_type: PositionType::Absolute,
//                                 left: Val::Px(250.),
//                                 top: Val::Px(100.),
//                                 width: Val::Px(300.),
//                                 height: Val::Px(100.),
//                                 display: Display::Flex,
//                                 flex_direction: FlexDirection::Column,
//                                 justify_content: JustifyContent::Center,
//                                 align_items: AlignItems::Center,
//                                 ..Default::default()
//                             },
//                             // background_color: BackgroundColor(Color::GREEN),
//                             ..Default::default()
//                         })
//                         .with_children(|parent| {
//                             parent.spawn((
//                                 TextBundle::from_sections([
//                                     TextSection::new(
//                                         "Food: ",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ),
//                                     TextSection::new(
//                                         "",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ),
//                                 ]),
//                                 SettlementFoodText,
//                             ));

//                             parent.spawn((
//                                 TextBundle::from_sections([
//                                     TextSection::new(
//                                         "Water: ",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ),
//                                     TextSection::new(
//                                         "",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ),
//                                 ]),
//                                 SettlementWaterText,
//                             ));

//                             parent.spawn((
//                                 TextBundle::from_sections([
//                                     TextSection::new(
//                                         "Wood: ",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ),
//                                     TextSection::new(
//                                         "",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ),
//                                 ]),
//                                 SettlementWoodText,
//                             ));
//                         });
//                 });

//             parent
//                 .spawn(NodeBundle {
//                     style: Style {
//                         position_type: PositionType::Absolute,
//                         width: Val::Px(780.),
//                         height: Val::Px(360.),
//                         left: Val::Px(10.),
//                         top: Val::Px(180.),
//                         ..Default::default()
//                     },
//                     // background_color: BackgroundColor(Color::ORANGE),
//                     ..Default::default()
//                 })
//                 .with_children(|parent| {
//                     parent
//                         .spawn(NodeBundle {
//                             style: Style {
//                                 position_type: PositionType::Absolute,
//                                 width: Val::Px(500.),
//                                 height: Val::Px(65.),
//                                 top: Val::Px(100.),
//                                 left: Val::Px(145.),
//                                 display: Display::Flex,
//                                 justify_content: JustifyContent::SpaceBetween,
//                                 ..Default::default()
//                             },
//                             // background_color: BackgroundColor(Color::PURPLE),
//                             ..Default::default()
//                         })
//                         .with_children(|parent| {
//                             parent
//                                 .spawn((
//                                     StoreFoodButton,
//                                     ButtonBundle {
//                                         style: Style {
//                                             width: Val::Px(150.0),
//                                             height: Val::Px(65.0),
//                                             justify_content: JustifyContent::Center,
//                                             align_items: AlignItems::Center,
//                                             ..Default::default()
//                                         },
//                                         background_color: BackgroundColor(Color::GRAY),
//                                         ..Default::default()
//                                     },
//                                 ))
//                                 .with_children(|parent| {
//                                     parent.spawn(TextBundle::from_section(
//                                         "Store 1 Food",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ));
//                                 });

//                             parent
//                                 .spawn((
//                                     StoreWaterButton,
//                                     ButtonBundle {
//                                         style: Style {
//                                             width: Val::Px(150.0),
//                                             height: Val::Px(65.0),
//                                             justify_content: JustifyContent::Center,
//                                             align_items: AlignItems::Center,
//                                             ..Default::default()
//                                         },
//                                         background_color: BackgroundColor(Color::GRAY),
//                                         ..Default::default()
//                                     },
//                                 ))
//                                 .with_children(|parent| {
//                                     parent.spawn(TextBundle::from_section(
//                                         "Store 1 Water",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ));
//                                 });

//                             parent
//                                 .spawn((
//                                     StoreWoodButton,
//                                     ButtonBundle {
//                                         style: Style {
//                                             width: Val::Px(150.0),
//                                             height: Val::Px(65.0),
//                                             justify_content: JustifyContent::Center,
//                                             align_items: AlignItems::Center,
//                                             ..Default::default()
//                                         },
//                                         background_color: BackgroundColor(Color::GRAY),
//                                         ..Default::default()
//                                     },
//                                 ))
//                                 .with_children(|parent| {
//                                     parent.spawn(TextBundle::from_section(
//                                         "Store 1 Wood",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ));
//                                 });
//                         });

//                     parent
//                         .spawn(NodeBundle {
//                             style: Style {
//                                 position_type: PositionType::Absolute,
//                                 width: Val::Px(500.),
//                                 height: Val::Px(65.),
//                                 top: Val::Px(175.),
//                                 left: Val::Px(145.),
//                                 display: Display::Flex,
//                                 justify_content: JustifyContent::SpaceBetween,
//                                 ..Default::default()
//                             },
//                             // background_color: BackgroundColor(Color::PURPLE),
//                             ..Default::default()
//                         })
//                         .with_children(|parent| {
//                             parent
//                                 .spawn((
//                                     TakeFoodButton,
//                                     ButtonBundle {
//                                         style: Style {
//                                             width: Val::Px(150.0),
//                                             height: Val::Px(65.0),
//                                             justify_content: JustifyContent::Center,
//                                             align_items: AlignItems::Center,
//                                             ..Default::default()
//                                         },
//                                         background_color: BackgroundColor(Color::GRAY),
//                                         ..Default::default()
//                                     },
//                                 ))
//                                 .with_children(|parent| {
//                                     parent.spawn(TextBundle::from_section(
//                                         "Take 1 Food",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ));
//                                 });

//                             parent
//                                 .spawn((
//                                     TakeWaterButton,
//                                     ButtonBundle {
//                                         style: Style {
//                                             width: Val::Px(150.0),
//                                             height: Val::Px(65.0),
//                                             justify_content: JustifyContent::Center,
//                                             align_items: AlignItems::Center,
//                                             ..Default::default()
//                                         },
//                                         background_color: BackgroundColor(Color::GRAY),
//                                         ..Default::default()
//                                     },
//                                 ))
//                                 .with_children(|parent| {
//                                     parent.spawn(TextBundle::from_section(
//                                         "Take 1 Water",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ));
//                                 });

//                             parent
//                                 .spawn((
//                                     TakeWoodButton,
//                                     ButtonBundle {
//                                         style: Style {
//                                             width: Val::Px(150.0),
//                                             height: Val::Px(65.0),
//                                             justify_content: JustifyContent::Center,
//                                             align_items: AlignItems::Center,
//                                             ..Default::default()
//                                         },
//                                         background_color: BackgroundColor(Color::GRAY),
//                                         ..Default::default()
//                                     },
//                                 ))
//                                 .with_children(|parent| {
//                                     parent.spawn(TextBundle::from_section(
//                                         "Take 1 Wood",
//                                         TextStyle {
//                                             font_size: 30.,
//                                             color: Color::WHITE,
//                                             ..Default::default()
//                                         },
//                                     ));
//                                 });
//                         });
//                 });
//         });
// }

// pub fn open_home_ui(
//     current_location: Res<CurrentLocation>,
//     mut open_home_ui_button_query: Query<
//         (&Interaction, &mut Visibility),
//         (
//             Changed<Interaction>,
//             (With<OpenHomeUIButton>, Without<HomeUI>),
//         ),
//     >,
//     mut home_ui_query: Query<&mut Visibility, (With<HomeUI>, Without<OpenHomeUIButton>)>,
// ) {
//     if let Ok((interaction, mut visibility)) = open_home_ui_button_query.get_single_mut() {
//         if matches!(*interaction, Interaction::Pressed) && current_location.0 == 0 {
//             *home_ui_query.single_mut() = Visibility::Visible;
//             *visibility = Visibility::Hidden;
//         }
//     }
// }

// pub fn close_home_ui(
//     close_home_ui_button_query: Query<
//         &Interaction,
//         (
//             Changed<Interaction>,
//             (
//                 With<CloseHomeUIButton>,
//                 Without<OpenHomeUIButton>,
//                 Without<HomeUI>,
//             ),
//         ),
//     >,
//     mut open_home_ui_button_query: Query<
//         &mut Visibility,
//         (
//             With<OpenHomeUIButton>,
//             Without<CloseHomeUIButton>,
//             Without<HomeUI>,
//         ),
//     >,
//     mut home_ui_query: Query<
//         &mut Visibility,
//         (
//             With<HomeUI>,
//             Without<OpenHomeUIButton>,
//             Without<CloseHomeUIButton>,
//         ),
//     >,
// ) {
//     if let Ok(interaction) = close_home_ui_button_query.get_single() {
//         if matches!(*interaction, Interaction::Pressed) {
//             *home_ui_query.single_mut() = Visibility::Hidden;
//             *open_home_ui_button_query.single_mut() = Visibility::Visible;
//         }
//     }
// }

// pub fn show_open_home_ui_button(
//     mut open_home_ui_button_query: Query<&mut Visibility, With<OpenHomeUIButton>>,
//     mut show_open_home_ui_events: EventReader<ShowOpenHomeUIButton>,
// ) {
//     for _ in show_open_home_ui_events.read() {
//         *open_home_ui_button_query.single_mut() = Visibility::Visible;
//     }
// }

// pub fn hide_open_home_ui_button(
//     mut open_home_ui_button_query: Query<&mut Visibility, With<OpenHomeUIButton>>,
//     mut hide_open_home_ui_events: EventReader<HideOpenHomeUIButton>,
// ) {
//     for _ in hide_open_home_ui_events.read() {
//         *open_home_ui_button_query.single_mut() = Visibility::Hidden;
//     }
// }

// pub fn store_and_take_resources(
//     store_food_button_query: Query<&Interaction, (Changed<Interaction>, With<StoreFoodButton>)>,
//     take_food_button_query: Query<&Interaction, (Changed<Interaction>, With<TakeFoodButton>)>,
//     store_water_button_query: Query<&Interaction, (Changed<Interaction>, With<StoreWaterButton>)>,
//     take_water_button_query: Query<&Interaction, (Changed<Interaction>, With<TakeWaterButton>)>,
//     store_wood_button_query: Query<&Interaction, (Changed<Interaction>, With<StoreWoodButton>)>,
//     take_wood_button_query: Query<&Interaction, (Changed<Interaction>, With<TakeWoodButton>)>,
//     mut settlement_resources: ResMut<SettlementResources>,
//     mut player_resources: ResMut<PlayerResources>,
//     mut resources_events: EventWriter<UpdateResources>,
// ) {
//     let mut resources_updated = false;

//     for interaction in &store_food_button_query {
//         if matches!(*interaction, Interaction::Pressed) {
//             if player_resources.food > 0 {
//                 settlement_resources.food += 1;
//                 player_resources.food -= 1;

//                 resources_updated = true;
//             }
//         }
//     }

//     for interaction in &take_food_button_query {
//         if matches!(*interaction, Interaction::Pressed) {
//             if settlement_resources.food > 0 {
//                 settlement_resources.food -= 1;
//                 player_resources.food += 1;

//                 resources_updated = true;
//             }
//         }
//     }

//     for interaction in &store_water_button_query {
//         if matches!(*interaction, Interaction::Pressed) {
//             if player_resources.water > 0 {
//                 settlement_resources.water += 1;
//                 player_resources.water -= 1;

//                 resources_updated = true;
//             }
//         }
//     }

//     for interaction in &take_water_button_query {
//         if matches!(*interaction, Interaction::Pressed) {
//             if settlement_resources.water > 0 {
//                 settlement_resources.water -= 1;
//                 player_resources.water += 1;

//                 resources_updated = true;
//             }
//         }
//     }

//     for interaction in &store_wood_button_query {
//         if matches!(*interaction, Interaction::Pressed) {
//             if player_resources.wood > 0 {
//                 settlement_resources.wood += 1;
//                 player_resources.wood -= 1;

//                 resources_updated = true;
//             }
//         }
//     }

//     for interaction in &take_wood_button_query {
//         if matches!(*interaction, Interaction::Pressed) {
//             if settlement_resources.wood > 0 {
//                 settlement_resources.wood -= 1;
//                 player_resources.wood += 1;

//                 resources_updated = true;
//             }
//         }
//     }

//     if resources_updated {
//         resources_events.send(UpdateResources {
//             player_food: Some(player_resources.food),
//             player_water: Some(player_resources.water),
//             player_wood: Some(player_resources.wood),
//             settlement_food: Some(settlement_resources.food),
//             settlement_water: Some(settlement_resources.water),
//             settlement_wood: Some(settlement_resources.wood),
//         });
//     }
// }
