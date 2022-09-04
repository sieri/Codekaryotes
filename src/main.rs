mod life;

use bevy::prelude::*;
use crate::life::LifePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LifePlugin)
        .run()
}

