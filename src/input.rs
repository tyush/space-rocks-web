use crate::{
    objects::Ship,
    physics::{angular::AngularMomentum, velocity::Velocity},
    SHIP_SPEED, SHIP_TURN_SPEED,
};
use bevy::prelude::*;

#[non_exhaustive]
pub enum UserInputs {
    Left,
    Right,
    Forward,
    Backward,
    Fire,
    Dodge,
    IgnoreMe, // allows imo cleaner input.system
}

pub fn input_parser(mut ship: Query<&mut Ship>, keyboard_input: Res<Input<KeyCode>>) {
    for mut s in ship.iter_mut() {
        s.inputs.clear();

        for k in keyboard_input.get_pressed() {
            s.inputs.push(match k {
                KeyCode::Left | KeyCode::A => UserInputs::Left,
                KeyCode::Right | KeyCode::D => UserInputs::Right,
                KeyCode::Up | KeyCode::W => UserInputs::Forward,
                KeyCode::Down | KeyCode::S => UserInputs::Backward,
                KeyCode::Space => UserInputs::Fire,
                KeyCode::LShift => UserInputs::Dodge,
                _ => UserInputs::IgnoreMe,
            })
        }
    }
}

pub fn movements_to_vel(
    mut ships: Query<(&Transform, &mut Ship, &mut Velocity, &mut AngularMomentum)>,
    time: Res<Time>,
) {
    for (trans, ship, mut vel, mut ang_mom) in ships.iter_mut() {
        // print!(
        //     "\r{:#?}                       ",
        //     ship
        // );
        for input in &ship.inputs {
            match *input {
                UserInputs::Left => {
                    // trans.rotate(Quat::from_axis_angle(Vec3::Z, 5.0 * time.delta_seconds()));
                    ang_mom.0 += SHIP_TURN_SPEED * time.delta_seconds();
                }
                UserInputs::Right => {
                    // trans.rotate(Quat::from_axis_angle(Vec3::Z, -5.0 * time.delta_seconds()));
                    ang_mom.0 -= SHIP_TURN_SPEED * time.delta_seconds();
                }
                UserInputs::Forward => {
                    // ubevy::translate_respect_rot(
                    //     &mut trans,
                    //     Vec3::Y * -SHIP_SPEED * time.delta_seconds(),
                    // );
                    let old_vel = vel.2;
                    *vel += Velocity::given_vec3(
                        trans.rotation * Vec3::new(0.0, -SHIP_SPEED * time.delta_seconds(), 0.0),
                        old_vel,
                    );
                    // *vel = Velocity::given_vec3(trans.rotation * Vec3::new(0.0, -200.0, 0.0));
                }
                UserInputs::IgnoreMe => (),
                _ => (),
            }
        }
    }
}
