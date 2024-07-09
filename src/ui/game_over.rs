use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    render::{color::Color, view::Visibility},
    text::TextStyle,
    ui::{node_bundles::TextBundle, PositionType, Style, Val},
};

use crate::{
    locations::location::{CurrentLocation, LocationId},
    PlayerResources,
};

#[derive(Component)]
pub struct GameOverText;

pub fn setup_game_over(mut commands: Commands) {
    let mut text = TextBundle::from_section(
        "GAME OVER",
        TextStyle {
            font_size: 90.,
            color: Color::WHITE,
            ..Default::default()
        },
    )
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(350.),
        left: Val::Px(550.),
        ..Default::default()
    });

    text.visibility = Visibility::Hidden;

    commands.spawn((GameOverText, text));
}

pub fn check_for_game_over(
    mut query: Query<&mut Visibility, With<GameOverText>>,
    current_location: Res<CurrentLocation>,
    player_resources: ResMut<PlayerResources>,
) {
    if (player_resources.food <= 0 || player_resources.water <= 0 || player_resources.wood <= 0)
        && current_location.0 != LocationId(0)
    {
        *query.single_mut() = Visibility::Visible;
    }
}
