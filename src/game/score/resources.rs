use bevy::{prelude::*, utils::HashMap};

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Debug)]
pub enum ScoreError {
    NoEntityWithName(String),
}

#[derive(Resource, Default)]
pub struct Score {
    pub data: HashMap<String, (Handle<Image>, u32)>,
}

impl Score {
    pub fn add_score_to(&mut self, who: &str, image: Handle<Image>) {
        let old_score = match self.data.get(who) {
            Some((_, score)) => *score,
            None => 0,
        };
        self.data.insert(who.to_string(), (image, old_score + 1));
    }

    pub fn get_score(&self, for_who: &str) -> Result<u32, ScoreError> {
        match self.data.get(for_who) {
            Some((_, score)) => Ok(*score),
            None => Err(ScoreError::NoEntityWithName(String::from(for_who))),
        }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(Handle<Image>, String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores { scores: Vec::new() }
    }
}
