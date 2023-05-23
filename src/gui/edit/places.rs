use std::collections::BTreeMap;
use bevy::{ecs::system::{CommandQueue, Spawn}, prelude::{Or, Entity, With, Parent, Children, QueryState, Without, World, DespawnRecursive}};
use eframe::{egui, epaint::Color32};
use crate::{world::{sim::SimulationData, place::{Settlement, Region, RegionBundle, SettlementBundle}, thing::Name}, gui::{EntityStringHashable, ecs::SpawnChild}};

pub(super) fn edit_places_ui(
    ui: &mut egui::Ui,
    state: &mut BTreeMap<String, String>,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    ui.horizontal(|ui| {
        if ui.button("New region").clicked() {
            queue.push(Spawn { bundle: RegionBundle::default() });
        }
        
        if ui.button("New settlement").clicked() {
            queue.push(Spawn { bundle: SettlementBundle::default() });
        }
    });

    ui.separator();

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
            recursively_create_ui(*root, &subnodes, queue, ui, world, &mut regions, &mut settlements);
        }
    });
}

fn recursively_create_ui(
    element: Entity,
    subnodes: &BTreeMap<Entity, Vec<Entity>>,
    queue: &mut CommandQueue,
    ui: &mut egui::Ui,
    world: &mut World,
    regions: &mut QueryState<(Entity, &mut Name, &mut Region), Without<Settlement>>,
    settlements: &mut QueryState<(Entity, &mut Name, &mut Settlement), Without<Region>>,
) {
    let header_title: String;

    match regions.get(world, element) {
        Ok((_, name, _)) => {
            header_title = name.0.clone();
        },
        Err(_) => {
            match settlements.get(world, element) {
                Ok((_, name, _)) => {
                    header_title = name.0.clone();
                },
                Err(_) => {
                    header_title = "Error! Open me!".to_string();
                },
            }
        },
    }

    egui::CollapsingHeader::new(format!("{} {:?}", header_title, element))
    .id_source(EntityStringHashable(element, "place_config".to_string()))
    .show(ui, |ui| {
        match regions.get_mut(world, element) {
            Ok((entity, mut name, mut region)) => {
                region_ui(queue, ui, entity, &mut *name, &mut *region);
            },
            Err(_) => {
                match settlements.get_mut(world, element) {
                    Ok((entity, mut name, mut settlement)) => {
                        settlement_ui(queue, ui, entity, &mut *name, &mut *settlement);
                    },
                    Err(_) => {
                        ui.label(egui::RichText::new("Something went wrong when querying the settlement entity.\nThis is a bug, and you should report it.").color(Color32::RED));
                    },
                }
            },
        }

        if subnodes.contains_key(&element) {
            ui.add_space(6.0);
            ui.label("Sub-regions and settlements");
            let children = &subnodes[&element];
            for child in children {
                recursively_create_ui(*child, &subnodes, queue, ui, world, regions, settlements);
            }
        }
    });
}

fn region_ui(
    queue: &mut CommandQueue,
    ui: &mut egui::Ui,
    entity: Entity,
    name: &mut Name,
    region: &mut Region,
) {
    ui.horizontal(|ui| {
        if ui.button("New child region").clicked() {
            queue.push(SpawnChild { bundle: RegionBundle::default(), parent: entity });
        }
        
        if ui.button("New child settlement").clicked() {
            queue.push(SpawnChild { bundle: SettlementBundle::default(), parent: entity });
        }

        if ui.button("Delete this object").clicked() {
            queue.push(DespawnRecursive { entity });
        }
    });

    ui.add_space(6.0);

    egui::Grid::new(EntityStringHashable(entity, "region_editor".to_string()))
    .show(ui, |ui| {
        ui.label("Name");
        ui.add(egui::TextEdit::singleline(&mut name.0).min_size(eframe::emath::Vec2::new(250.0, 0.0)));
        ui.end_row();
    });
}

fn settlement_ui(
    queue: &mut CommandQueue,
    ui: &mut egui::Ui,
    entity: Entity,
    name: &mut Name,
    settlement: &mut Settlement,
) {
    if ui.button("Delete this object").clicked() {
        queue.push(DespawnRecursive { entity });
    }

    ui.add_space(6.0);

    egui::Grid::new(EntityStringHashable(entity, "settlement_editor".to_string()))
    .show(ui, |ui| {
        ui.label("Name");
        ui.add(egui::TextEdit::singleline(&mut name.0).min_size(eframe::emath::Vec2::new(250.0, 0.0)));
        ui.end_row();

        ui.label("Population");
        ui.add(egui::DragValue::new(&mut settlement.population));
        ui.end_row();
    });
}