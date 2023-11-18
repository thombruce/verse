use bevy::prelude::*;

#[derive(Resource, Deref)]
pub struct Score(pub u32);

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
    }
}
