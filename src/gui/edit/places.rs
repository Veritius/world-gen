use std::collections::{BTreeMap, BTreeSet};
use bevy::{ecs::system::CommandQueue, prelude::{Entity, Parent, Children, With, Without}};
use eframe::egui;
use either::Either::{Left, self, Right};
use petgraph::Graph;
use crate::world::{sim::SimulationData, place::{Settlement, Region}, thing::Name};

pub(super) fn edit_places_ui(
    ui: &mut egui::Ui,
    state: &mut BTreeMap<String, String>,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    let world = &mut sim.app.world;

    let mut graph: Graph<Either<(), Entity>, ()> = Graph::new();
    let root = graph.add_node(Left(()));

    let mut region_query = world.query_filtered::<(Entity, &Region, &Name, Option<&Parent>, Option<&Children>), Without<Settlement>>();
    let mut settlement_query = world.query_filtered::<(Entity, &Settlement, &Name, Option<&Parent>), Without<Region>>();

    let mut nodes = BTreeMap::new();

    // Add nodes to graph
    for (entity, _, _, _, _) in region_query.iter(&world) {
        nodes.insert(entity, graph.add_node(Right(entity)));
    }

    for (entity, _, _, _) in settlement_query.iter(&world) {
        nodes.insert(entity, graph.add_node(Right(entity)));
    }

    // Add edges
    for (entity, _, _, parent, children) in region_query.iter(&world) {
        let this = nodes[&entity];
        // Parent edges
        if parent.is_some() {
            let parent = nodes.get(&parent.unwrap());
            if parent.is_some() {
                // Child of parent
                graph.add_edge(*parent.unwrap(), this, ());
            } else {
                // Child of root
                graph.add_edge(root, this, ());
            }
        }
        // Child edges
        if children.is_some() {
            let children = children.unwrap();
            for child in children.iter() {
                let child = nodes.get(child);
                if child.is_some() {
                    // Parent of child
                    graph.add_edge(this, *child.unwrap(), ());
                }
            }
        }
    }

    for (entity, _, _, parent) in settlement_query.iter(&world) {
        let this = nodes[&entity];
        if parent.is_some() {
            let parent = nodes.get(&parent.unwrap().get());
            if parent.is_some() {
                // Child of parent
                graph.add_edge(*parent.unwrap(), this, ());
            } else {
                // Child of root
                graph.add_edge(root, this, ());
            }
        }
    }
}