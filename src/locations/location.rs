use bevy::{
    asset::Handle,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    render::texture::Image,
    sprite::SpriteBundle,
    transform::components::Transform,
    utils::HashMap,
};

use crate::{
    overworld::entities::{
        berry_bush::{BerryBush, BerryBushBundle, BerryBushState},
        change_location_zone::ChangeLocationZoneBundle,
        player::Player,
        stick::StickBundle,
        stockpile::StockpileBundle,
        tree::TreeBundle,
        water_pool::WaterPoolBundle,
    },
    party_resources::{UpdatePartyFoodEvent, UpdatePartyWaterEvent},
};

#[derive(Component)]
pub struct LocationScene;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct LocationId(pub u32);

#[derive(Resource)]
pub struct CurrentLocation(pub LocationId);

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct LocationEntityId(pub u32);

pub struct Location {
    pub background: SpriteBundle,
    pub sticks: Option<HashMap<LocationEntityId, StickBundle>>,
    pub trees: Option<HashMap<LocationEntityId, TreeBundle>>,
    pub berry_bushes: Option<HashMap<LocationEntityId, BerryBushBundle>>,
    pub water_pools: Option<HashMap<LocationEntityId, WaterPoolBundle>>,
    pub change_location_zones: Option<HashMap<LocationEntityId, ChangeLocationZoneBundle>>,
    pub stockpile: Option<StockpileBundle>,
}

#[derive(Resource, Default)]
pub struct Locations(pub HashMap<LocationId, Location>);

#[derive(Resource, Default)]
pub struct LocationEntities(pub HashMap<LocationEntityId, Entity>);

#[derive(Event)]
pub struct ChangeLocationEvent(pub LocationId);

#[derive(Event)]
pub struct SpawnLocationEvent(pub LocationId);

#[derive(Event)]
pub struct DespawnLocationEvent(pub LocationId);

pub fn spawn_location(
    mut commands: Commands,
    mut spawn_location_events: EventReader<SpawnLocationEvent>,
    mut location_scene_query: Query<Entity, With<LocationScene>>,
    locations: Res<Locations>,
    mut location_entities: ResMut<LocationEntities>,
) {
    for ev in spawn_location_events.read() {
        location_entities.0.clear();

        let location = &locations.0[&ev.0];

        let mut entities = vec![];

        entities.push(commands.spawn(location.background.clone()).id());

        if let Some(sticks) = &location.sticks {
            for (stick_id, stick_bundle) in sticks {
                let stick = commands.spawn(stick_bundle.clone()).id();
                location_entities.0.insert(stick_id.clone(), stick);
                entities.push(stick);
            }
        }

        if let Some(trees) = &location.trees {
            for (tree_id, tree_bundle) in trees {
                let tree = commands.spawn(tree_bundle.clone()).id();
                location_entities.0.insert(tree_id.clone(), tree);
                entities.push(tree);
            }
        }

        if let Some(berry_bushes) = &location.berry_bushes {
            for (berry_bush_id, berry_bush_bundle) in berry_bushes {
                let berry_bush = commands.spawn(berry_bush_bundle.clone()).id();
                location_entities
                    .0
                    .insert(berry_bush_id.clone(), berry_bush);
                entities.push(berry_bush);
            }
        }

        if let Some(water_pools) = &location.water_pools {
            for (water_pool_id, water_pool_bundle) in water_pools {
                let water_pool = commands.spawn(water_pool_bundle.clone()).id();
                location_entities
                    .0
                    .insert(water_pool_id.clone(), water_pool);
                entities.push(water_pool);
            }
        }

        if let Some(change_location_zones) = &location.change_location_zones {
            for (change_location_zone_id, change_location_zone_bundle) in change_location_zones {
                let change_location_zone = commands.spawn(change_location_zone_bundle.clone()).id();
                location_entities
                    .0
                    .insert(change_location_zone_id.clone(), change_location_zone);
                entities.push(change_location_zone);
            }
        }

        if let Some(stockpile) = &location.stockpile {
            let stockpile = commands.spawn(stockpile.clone()).id();
            entities.push(stockpile);
        }

        let location_scene = location_scene_query.single_mut();

        commands
            .entity(location_scene)
            .push_children(entities.as_slice());
    }
}

fn save_location_state(
    commands: &mut Commands,
    location: &mut Location,
    location_entities: &mut ResMut<LocationEntities>,
    berry_bush_query: &Query<(&BerryBushState, &Handle<Image>), With<BerryBush>>,
) {
    if let Some(sticks) = location.sticks.clone() {
        for stick_id in sticks.keys() {
            if commands.get_entity(location_entities.0[stick_id]).is_none() {
                location.sticks.as_mut().unwrap().remove(stick_id);
            }
        }
    }

    if let Some(berry_bushes) = location.berry_bushes.clone() {
        for berry_bush_id in berry_bushes.keys() {
            let (berry_bush_state, berry_bush_texture) = berry_bush_query
                .get(location_entities.0[berry_bush_id])
                .unwrap();

            location
                .berry_bushes
                .as_mut()
                .unwrap()
                .get_mut(berry_bush_id)
                .unwrap()
                .state = berry_bush_state.clone();

            location
                .berry_bushes
                .as_mut()
                .unwrap()
                .get_mut(berry_bush_id)
                .unwrap()
                .sprite
                .texture = berry_bush_texture.clone();
        }
    }
}

pub fn despawn_location(
    mut commands: Commands,
    mut despawn_location_events: EventReader<DespawnLocationEvent>,
    mut location_scene_query: Query<Entity, With<LocationScene>>,
    mut locations: ResMut<Locations>,
    mut location_entities: ResMut<LocationEntities>,
    berry_bush_query: Query<(&BerryBushState, &Handle<Image>), With<BerryBush>>,
) {
    for ev in despawn_location_events.read() {
        let location = locations.0.get_mut(&ev.0).unwrap();

        save_location_state(
            &mut commands,
            location,
            &mut location_entities,
            &berry_bush_query,
        );

        commands
            .entity(location_scene_query.single_mut())
            .despawn_descendants();

        location_entities.0.clear();
    }
}

pub fn change_location(
    mut change_location_events: EventReader<ChangeLocationEvent>,
    mut spawn_location_events: EventWriter<SpawnLocationEvent>,
    mut despawn_location_events: EventWriter<DespawnLocationEvent>,
    mut current_location: ResMut<CurrentLocation>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut update_food_events: EventWriter<UpdatePartyFoodEvent>,
    mut update_water_events: EventWriter<UpdatePartyWaterEvent>,
) {
    for ev in change_location_events.read() {
        despawn_location_events.send(DespawnLocationEvent(current_location.0.clone()));
        spawn_location_events.send(SpawnLocationEvent(ev.0.clone()));

        update_food_events.send(UpdatePartyFoodEvent(-1));
        update_water_events.send(UpdatePartyWaterEvent(-1));

        current_location.0 = ev.0.clone();

        let mut transform = player_transform_query.single_mut();
        transform.translation.x = 0.;
        transform.translation.y = 0.;
    }
}
