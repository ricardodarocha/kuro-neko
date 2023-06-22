use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::{resources::*, systems::*};
use crate::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod resources;
pub mod systems;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<ScoreUpdateEvent>()
            // Resources
            .init_resource::<HighScores>()
            // Enter State Systems
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(update_highscores)
            .add_system(high_scores_updated)
            .add_system(update_score.in_set(OnUpdate(AppState::Game)))
            // Exit State Systems
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}

// Events
pub struct ScoreUpdateEvent {
    pub name: String,
    pub event_type: ScoreEventType,
}

pub enum ScoreEventType {
    NewHighscore(u32),
    ReachedMilestone(u32),
}

impl ScoreUpdateEvent {
    pub fn new(name: String, event_type: ScoreEventType) -> Self {
        ScoreUpdateEvent { name, event_type }
    }
}

impl ScoreEventType {
    pub fn get_score(&self) -> u32 {
        match self {
            ScoreEventType::NewHighscore(score) => *score,
            ScoreEventType::ReachedMilestone(score) => *score,
        }
    }
}
