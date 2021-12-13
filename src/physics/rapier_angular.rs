use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn set_velocity_ship(
    mut items: Query<(&mut RigidBodyBundle)>,
);