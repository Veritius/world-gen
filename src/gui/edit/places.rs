use std::collections::BTreeMap;
use bevy::{ecs::system::CommandQueue, prelude::{Or, Entity, With, Parent, Children, QueryState, Without, World}};
use eframe::egui;
use either::Either;
use crate::{world::{sim::SimulationData, place::{Settlement, Region}, thing::Name}, gui::EntityStringHashable};

pub(super) fn edit_places_ui(
    ui: &mut egui::Ui,
    state: &mut BTreeMap<String, String>,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    let world = &mut sim.app.world;

    let mut all_nodes = world.query_filtered::<(Entity, Option<&Parent>, Option<&Children>), Or<(With<Region>, With<Settlement>)>>();

    let ilen = all_nodes.iter(&world).len();
    let mut roots: Vec<Entity> = Vec::with_capacity(ilen);
    let mut subnodes: BTreeMap<Entity, Vec<Entity>> = BTreeMap::new();

    for (node, parent, children) in all_nodes.iter(&world) {
        if parent.is_none() {
            roots.push(node);
        }

        if children.is_some() {
            let children = children.unwrap().iter();
            let mut set = Vec::with_capacity(children.len());
            for child in children {
                set.push(*child);
            }

            set.sort();
            subnodes.insert(node, set);
        }
    }

    let mut regions = world.query_filtered::<(Entity, &mut Name, &mut Region), Without<Settlement>>();
    let mut settlements = world.query_filtered::<(Entity, &mut Name, &mut Settlement), Without<Region>>();

    egui::ScrollArea::both()
    .id_source("places_scroll_area")
    .auto_shrink([false, false])
    .show(ui, |ui| {
        for root in &roots {
            recursively_create_ui(*root, &subnodes, ui, world, &mut regions, &mut settlements);
        }
    });
}

fn recursively_create_ui(
    element: Entity,
    subnodes: &BTreeMap<Entity, Vec<Entity>>,
    ui: &mut egui::Ui,
    world: &mut World,
    regions: &mut QueryState<(Entity, &mut Name, &mut Region), Without<Settlement>>,
    settlements: &mut QueryState<(Entity, &mut Name, &mut Settlement), Without<Region>>,
) {
    if !subnodes.contains_key(&element) { return; }
    let children = &subnodes[&element];

    let mut header_title: String;
    let mut ui_fn: Box<dyn FnOnce(&mut egui::Ui)>;

    match regions.get_mut(world, element) {
        Ok((entity, mut name, mut region)) => {
            header_title = name.0.clone();
            ui_fn = Box::new(region_ui);
        },
        Err(_) => {},
    }

    match settlements.get_mut(world, element) {
        Ok((entity, mut name, mut settlement)) => {
            header_title = name.0.clone();
            ui_fn = Box::new(settlement_ui);
        },
        Err(_) => {
            header_title = "Something went very wrong".to_string();
            ui_fn = Box::new(|ui|{});
        },
    }

    egui::CollapsingHeader::new(format!("{} {:?}", header_title, element))
    .id_source(EntityStringHashable(element, "place_config".to_string()))
    .show(ui, ui_fn);

    egui::CollapsingHeader::new("Children")
    .id_source(EntityStringHashable(element, "children_elements".to_string()))
    .show(ui, |ui| {
        for child in children {
            recursively_create_ui(*child, &subnodes, ui, world, regions, settlements);
        }
    });
}

fn region_ui(
    ui: &mut egui::Ui,
) {
    ui.label("Region");
}

fn settlement_ui(
    ui: &mut egui::Ui,
) {
    ui.label("Settlement");
}