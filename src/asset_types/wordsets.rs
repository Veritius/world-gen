use bevy::{prelude::*, reflect::{TypeUuid, TypePath}};
use serde::Deserialize;

/// A set of strings.
#[derive(Deserialize, TypeUuid, TypePath)]
#[uuid="77449be9-8237-4a5b-8f1b-64e06cf022ed"]
pub struct WordSet(pub Vec<String>);

/// A set of strings with attached random weighting.
#[derive(Deserialize, TypeUuid, TypePath)]
#[uuid="1018ea51-a3e2-4be9-8d35-e803bba2241b"]
pub struct WeightedWordSet(pub Vec<(u16, String)>);