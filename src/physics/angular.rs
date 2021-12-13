use bevy::prelude::*;

#[derive(Debug)]
pub struct AngularMomentum(pub f32, pub f32); // note: second value is DECAY!

pub fn angular_momentum(mut items: Query<(&mut Transform, &mut AngularMomentum)>, time: Res<Time>) {
    for (mut trans, mut ang_mom) in items.iter_mut() {
        trans.rotate(Quat::from_rotation_z(ang_mom.0 * time.delta_seconds()));
        ang_mom.0 *= 1.0 - ang_mom.1 * time.delta_seconds();
        if ang_mom.0.abs() < 0.0000000001 {
            ang_mom.0 = 0.0;
        }
        // println!("Calcing {:?}", ang_mom);
    }
}
