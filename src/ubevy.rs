use bevy::prelude::*;

pub fn translate(transform: &mut Transform, shift: Vec3) {
    transform.translation += shift;
}

pub fn translate_respect_rot(transform: &mut Transform, shift: Vec3) {
    let new_shift = transform.rotation * shift;

    transform.translation += new_shift;
}

pub fn gen_points_in_between_points(from: (f32, f32), to: (f32, f32)) -> Vec<(f32, f32)> {
    let mut points: Vec<(f32, f32)> = Vec::new();

    // see https://www.desmos.com/calculator/qnima15c2v
    let p1 = from.0 - to.0;
    let p2 = from.1 - to.1;
    let p3: f32 = 0.0;
    let p4: f32 = 0.0;

    // gotta love functional programming
    let funct = |x: f32| ((p2 - p4) / (p1 - p3)) * (x - p1) + p2;

    let l = ((p1 - p3).powi(2) + (p2 - p4).powi(2)).sqrt();

    let s = l.ceil();

    let d = (p1 - p3) / s;

    for i in (1..).take_while(|i| *i <= (s as u32)) {
        points.push(((d * (i as f32)) + to.0, funct(d * (i as f32)) + to.1));
    }

    // info!("Using pointlist {:?} for {:?} to {:?}", points, from, to);

    points
}
