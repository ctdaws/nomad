use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        event::{Event, EventReader},
        query::With,
        system::{Commands, Query},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::{Text, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        PositionType, Style, Val,
    },
};

#[derive(Component)]
pub struct Encounter;

#[derive(Component)]
pub struct EncounterText;

#[derive(Event)]
pub struct UpdateEncounter(pub String);

#[derive(Bundle)]
pub struct EncounterBundle {
    pub marker: Encounter,
    pub node: NodeBundle,
}

pub fn setup_encounter(mut commands: Commands) {
    let encounter = commands
        .spawn(EncounterBundle {
            marker: Encounter,
            node: NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(1000.),
                    height: Val::Px(200.),
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .id();

    let text = commands
        .spawn((
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
        ))
        .id();

    commands.entity(encounter).push_children(&[text]);
}

pub fn update_encounter_text(
    mut ev_reader: EventReader<UpdateEncounter>,
    mut query: Query<&mut Text, With<EncounterText>>,
) {
    for ev in ev_reader.read() {
        let mut text = query.single_mut();
        text.sections[0].value = ev.0.clone();
    }
}
