use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Clone, Debug, Deserialize, TypeUuid)]
#[uuid = "a51952db-28e3-4bac-ba92-0d3c90921985"]
pub struct DogNames {
    pub first_names: Vec<String>,
    pub last_names: Vec<String>,
    pub nicknames: Vec<String>,
}