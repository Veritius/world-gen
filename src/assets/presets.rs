use bevy::reflect::{TypeUuid, TypePath};
use serde::Deserialize;

/// A preset of a faction.
#[derive(Deserialize, TypeUuid, TypePath)]
#[uuid="3d9deaf4-336d-4ae4-96a4-d511acee86b6"]
pub struct FactionPreset {
    pub name: String,
}

/// A preset of a species.
#[derive(Deserialize, TypeUuid, TypePath)]
#[uuid="14ad6b7e-bcda-4980-948b-1f36f36267bb"]
pub struct SpeciesPreset {
    pub name: String,
}