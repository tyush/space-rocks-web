use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ubevy;



#[derive(Debug, Serialize, Deserialize)]
pub struct RaceMap {
    pub track_points: Vec<TrackPoint>,
    pub track_points_other: Vec<TrackPoint>,
    pub finish_line: ((f32, f32), (f32, f32)),
}

impl RaceMap {
    pub fn put_into_world(
        &self,
        commands: &mut Commands,
        handle: Handle<ColorMaterial>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        let mut rails = Vec::new();

        for index in 0..self.track_points.len()-1 {
            let curr_point = self.track_points[index];
            let next_point = self.track_points[index + 1];

            // if let None = next_point {
            //     break;
            // }

            // let curr_point = curr_point.unwrap();
            // let next_point = next_point.unwrap();

            // let offset_curr = RaceMap::create_offset_point(curr_point);
            // let offset_next = RaceMap::create_offset_point(next_point);

            rails.push(curr_point.gen_bar_to_point(&next_point, handle.clone(), &mut meshes));
            // rails.push(offset_curr.gen_bar_to_point(&offset_next, handle.clone(), &mut meshes));
        }

        for index in 0..self.track_points_other.len()-1 {
            let curr_point = self.track_points_other[index];
            let next_point = self.track_points_other[index + 1];

            // if let None = next_point {
            //     break;
            // }

            // let curr_point = curr_point.unwrap();
            // let next_point = next_point.unwrap();

            // let offset_curr = RaceMap::create_offset_point(curr_point);
            // let offset_next = RaceMap::create_offset_point(next_point);

            rails.push(curr_point.gen_bar_to_point(&next_point, handle.clone(), &mut meshes));
            // rails.push(offset_curr.gen_bar_to_point(&offset_next, handle.clone(), &mut meshes));
        }

        // spawn backguard to prevent player from being
        // able to just go around the track
        // these values need to reflect server/src/Lib,java:X_OFFSET
        let blocker_left = (TrackPoint(-700.0, 0.0), TrackPoint(-700.0, -300.0));
        let blocker_right = (TrackPoint(700.0, 0.0), TrackPoint(700.0, -300.0));
        rails.push(blocker_left.0.gen_bar_to_point(&blocker_left.1, handle.clone(), &mut meshes)); //  |
        rails.push(blocker_right.0.gen_bar_to_point(&blocker_right.1, handle.clone(), &mut meshes)); //  |    |
        rails.push(blocker_left.1.gen_bar_to_point(&blocker_right.1, handle.clone(), &mut meshes)); //  |____|

        // commands.spawn_batch(rails);
        for r in rails {
            commands.spawn_bundle(r.0).insert(r.1);
        }
    }

    pub fn clone_me(&self) -> RaceMap {
        RaceMap {
            track_points: self.track_points.to_vec(),
            track_points_other: self.track_points_other.to_vec(),
            finish_line: self.finish_line.to_owned(),
        }
    }

    // fn create_offset_point(point: &TrackPoint) -> TrackPoint {
    //   TODO: add actually good implementation of an offset point
    //     // see https://www.desmos.com/calculator/olff0hl6yu
    //     TrackPoint(
    //         point.0.powi(2).atan() * (point.0 * 1.2),
    //         point.1.powi(2).atan() * (point.1 * 1.2)
    //     )
    // }
}

#[derive(Debug, Default, Reflect)]
pub struct TrackRail {
    pos: (f32, f32),
    pub length: f32,
    rot: f32,
    pub subpoints: Vec<(f32, f32)>,
}

impl TrackRail {
    pub fn from_len_deg(
        pos: (f32, f32),
        length: f32,
        rot: f32,
        from_to: (&TrackPoint, &TrackPoint),
        color: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> (bevy::prelude::SpriteBundle, TrackRail) {
        let mut transform = Transform::from_xyz(pos.0, pos.1, 0.0); // * length);
                                                                    // transform.translation = Vec3::new(pos.0 - (length/2.0), pos.1 - (length/2.0), 0.0);
        transform.rotation = Quat::from_rotation_z(rot);

        let sprite = SpriteBundle {
            material: color,
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Quad {
                size: Vec2::new(length, 1.0),
                flip: false,
            })),
            sprite: Sprite::new(Vec2::new(length, 1.0)),
            transform,
            ..Default::default()
        };

        let points: Vec<(f32, f32)> = ubevy::gen_points_in_between_points(
            (from_to.0 .0, from_to.0 .1),
            (from_to.1 .0, from_to.1 .1),
        );

        // points
            // .iter()
            // .for_each(|(x, y)| info!("Calcing collision for {}, {}", x, y));

        // {
        //     // see https://www.desmos.com/calculator/qnima15c2v
        //     let point_2: (f32, f32) = (0.0, 0.0);
        //     let point_1: (f32, f32) = (from_to.0.0 - from_to.1.0, from_to.0.1 - from_to.1.1);

        //     let slope = (point_1.0 - point_2.0)/(point_1.1 - point_2.1);
        //     let x_offset = point_1.0;
        //     let len = (point_1.0.powi(2) + point_1.1.powi(2)).sqrt();

        //     let distance: f32 = (POINTS_PER_UNIT * len).ceil();

        //     let distance_between_point = point_1.0 / distance;

        //     for i in 1..=(distance_between_point.ceil() as u32) {
        //         info!("Checking {:?} for collision", ((i as f32) + point_1.0, (slope * ((i as f32) - point_1.0)) + point_1.1));
        //         points.push(((i as f32) + x_offset, (slope * ((i as f32) - point_1.0)) + point_1.1));
        //     }

        // }

        (
            sprite,
            TrackRail {
                pos,
                length,
                rot,
                subpoints: points,
            },
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct TrackPoint(pub f32, pub f32);

impl TrackPoint {
    pub fn gen_bar_to_point(
        &self,
        other: &TrackPoint,
        handle: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> (SpriteBundle, TrackRail) {
        // commands.insert_resource()
        // slope to degrees; see https://www.desmos.com/calculator/vihl4hiveh
        let degree = ((self.1 - other.1) / (self.0 - other.0)).atan(); // * (180.0 / PI); // unit conversion: radians to euler degrees
                                                                       // nvm lol radians are better

        // find distance between two points
        let length = ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2))
            .sqrt()
            .sqrt();
        #[allow(unused_doc_comments)]
        /// expanded
        ///
        /// side2 |\ result
        ///       |_\     (hypotenuse)
        ///       side1
        ///
        /// {
        ///     let side1 = (self.0 - other.0);
        ///     let side2 = (self.1 - other.1);
        ///     (side1.powi(2) + side2.powi(2)).sqrt()
        /// }
        let center_point = ((self.0 + other.0) / 2.0, (self.1 + other.1) / 2.0);
        println!(
            "Making line with origin {:?} and rot {:?} from points {:?} and {:?}",
            center_point, degree, self, other
        );

        TrackRail::from_len_deg(center_point, length, degree, (self, other), handle, meshes)
    }
}
