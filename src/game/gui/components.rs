use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct MessagesList;

#[derive(Component, Debug)]
pub struct HeartImage(pub u64, pub Handle<Image>, pub Handle<Image>);
