use bevy::prelude::*;

pub fn translate(
    transform: &mut Transform,
    shift: Vec3
) {
    transform.translation += shift;
}

pub fn translate_respect_rot(
    transform: &mut Transform,
    shift: Vec3
) {
    let new_shift = transform.rotation * shift;

    transform.translation += new_shift;
}