use bevy::prelude::*;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Use this plugin to determine proper system ordering.
        //       Make use of chaining and buffering where necessary.
        //       Make sensible use of SystemSets, remembering that
        //       there also exist schedules which will run in this
        //       order:
        //
        //       - PreStartup
        //       - Startup
        //       - PostStartup
        //       - First
        //       - PreUpdate
        //       - OnEnter
        //       - OnTransition
        //       - OnExit
        //       - FixedUpdate
        //       - Update
        //       - PostUpdate
        //       - Last
        //
        //       Note that FixedUpdate is a special case and not
        //       guaranteed to run every tick. It will run N times
        //       determined by the amount of time that has elapsed
        //       since it was last run.
    }
}
