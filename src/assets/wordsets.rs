use bevy::reflect::{TypeUuid, TypePath};
use serde::Deserialize;

/// A set of words or phrases.
#[derive(Deserialize, TypeUuid, TypePath)]
#[uuid="77449be9-8237-4a5b-8f1b-64e06cf022ed"]
pub struct WordSet(pub Vec<String>);

impl WordSet {
    pub fn pick(&self) -> Option<&str> {
        fastrand::choice(&self.0).map(|z| z.as_str())
    }
}