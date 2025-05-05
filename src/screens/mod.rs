#![doc = r#"
All the main screen states and transitions
"#]
use bevy::prelude::*;

mod credits;
mod gameplay;
mod loading;
mod splash;
mod title;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default, Reflect)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Gameplay,
    //GameOver,
}

pub fn plugin(app: &mut App) {
    app.register_type::<Screen>();

    app.init_state::<Screen>();

    // things automatically removed on state transition
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        gameplay::plugin,
        credits::plugin,
    ));
}
