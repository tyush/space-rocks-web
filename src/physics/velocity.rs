use std::ops::AddAssign;

use bevy::prelude::*;

use crate::ubevy;

#[derive(Debug)]
pub struct Velocity(pub f32, pub f32, pub f32); // note that the third value is DECAY, NOT Z

impl Velocity {
    pub fn given_vec3(vec: Vec3, friction: f32) -> Self {
        Velocity(vec.x, vec.y, friction)
    }

    pub fn scale(&mut self, scale: f32) {
        self.0 *= scale;
        self.1 *= scale;
    }
}

impl AddAssign for Velocity {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        let avg = self.2 + rhs.2;
        self.2 = avg / 2.0;
    }
}

pub fn velocity(mut items: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>) {
    // return ();
    for (mut trans, mut vel) in items.iter_mut() {
        // ubevy::translate_respect_rot(&mut trans, Vec3::new(vel.0 * time.delta_seconds(), vel.1 * time.delta_seconds(), 0.0));
        ubevy::translate(
            &mut trans,
            Vec3::new(
                vel.0 * time.delta_seconds(),
                vel.1 * time.delta_seconds(),
                0.0,
            ),
        );
        let frict = vel.2;
        vel.scale(1.0 - frict * time.delta_seconds());
        if vel.0.abs() + vel.1.abs() < 0.0000001 {
            *vel = Velocity(0.0, 0.0, vel.2);
        }
    }
}
