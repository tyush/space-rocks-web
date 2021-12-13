use bevy::prelude::*;
// use bevy_inspector_egui::Inspectable;
use ron;
use serde::{Deserialize, Serialize};
use bevy_rapier2d::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RaceMap {
    pub track_points: Vec<TrackPoint>,
    pub track_points_other: Vec<TrackPoint>,
    pub finish_line: ((f32, f32), (f32, f32)),
}

impl RaceMap {
    pub fn put_into_world(&self, commands: &mut Commands, handle: Handle<ColorMaterial>, mut meshes: ResMut<Assets<Mesh>>) {
        
        let mut rails = Vec::new();
        
        for index in 0..self.track_points.len() {
            let curr_point = self.track_points.get(index);
            let next_point = self.track_points.get(index + 1);

            if let None = next_point {
                break
            }

            let curr_point = curr_point.unwrap();
            let next_point = next_point.unwrap();

            // let offset_curr = RaceMap::create_offset_point(curr_point);
            // let offset_next = RaceMap::create_offset_point(next_point);

            rails.push(curr_point.gen_bar_to_point(next_point, handle.clone(), &mut meshes));
            // rails.push(offset_curr.gen_bar_to_point(&offset_next, handle.clone(), &mut meshes));
        }

        for index in 0..self.track_points_other.len() {
            let curr_point = self.track_points_other.get(index);
            let next_point = self.track_points_other.get(index + 1);

            if let None = next_point {
                break
            }

            let curr_point = curr_point.unwrap();
            let next_point = next_point.unwrap();

            // let offset_curr = RaceMap::create_offset_point(curr_point);
            // let offset_next = RaceMap::create_offset_point(next_point);

            rails.push(curr_point.gen_bar_to_point(next_point, handle.clone(), &mut meshes));
            // rails.push(offset_curr.gen_bar_to_point(&offset_next, handle.clone(), &mut meshes));
        }

        commands.spawn_batch(rails);
    }

    pub fn clone_me(&self) -> RaceMap {
        RaceMap {
            track_points: self.track_points.to_vec(),
            track_points_other: self.track_points_other.to_vec(),
            finish_line: self.finish_line.to_owned()
        }
    }

    fn create_offset_point(point: &TrackPoint) -> TrackPoint {
        // see https://www.desmos.com/calculator/olff0hl6yu
        TrackPoint(
            point.0.powi(2).atan() * (point.0 * 1.2),
            point.1.powi(2).atan() * (point.1 * 1.2)
        )
    }
}

pub fn map_from_ron(file: &str) -> Result<RaceMap, ron::Error> {
    ron::from_str::<RaceMap>(file)
}

#[derive(Debug, Default, Reflect, Clone, Copy)]
pub struct TrackRail {
    pos: (f32, f32),
    length: f32,
    rot: f32
}

impl TrackRail {
    pub fn from_len_deg(
        pos: (f32, f32),
        length: f32,
        rot: f32,
        color: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>
    ) -> (bevy::prelude::SpriteBundle, ColliderBundle) {
        let mut transform = Transform::from_xyz(pos.0, pos.1, 0.0); // * length);
        // transform.translation = Vec3::new(pos.0 - (length/2.0), pos.1 - (length/2.0), 0.0);
        transform.rotation = Quat::from_rotation_z(rot);

        let sprite = SpriteBundle {
            material: color,
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Quad { size: Vec2::new(length, 1.0), flip: false})),
            sprite: Sprite::new(Vec2::new(length, 1.0)),
            transform, 
            ..Default::default()
        };

        (
            sprite
            , ColliderBundle {
                shape: ColliderShape::cuboid(length, 1.0),
                collider_type: ColliderType::Solid,
                position: (Vec2::new(pos.0, pos.1), 0.4).into(),
                material: ColliderMaterial { friction: 0.2, restitution: 0.3, ..Default::default() },
                mass_properties: ColliderMassProps::Density(2.0),
                ..Default::default()
            })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct TrackPoint(pub f32, pub f32);

impl TrackPoint {
    pub fn gen_bar_to_point(
        &self,
        other: &TrackPoint,
        handle: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>
    ) -> (SpriteBundle, ColliderBundle) {
        // commands.insert_resource()
        // slope to degrees; see https://www.desmos.com/calculator/vihl4hiveh
        let degree = ((self.1 - other.1) / (self.0 - other.0)).atan(); // * (180.0 / PI); // unit conversion: radians to euler degrees
                                                                           // nvm lol radians are better

        // find distance between two points
        let length = (
            (self.0 - other.0).powi(2)
            + (self.1 - other.1).powi(2)
        ).sqrt().sqrt();
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
        #[warn(unused_doc_comments)]
        
        let center_point = ((self.0 + other.0) / 2.0, (self.1 + other.1) / 2.0);
        println!("Making line with origin {:?} and rot {:?} from points {:?} and {:?}", center_point, degree, self, other);

        TrackRail::from_len_deg(
            center_point,
            length,
            degree,
            handle,
            meshes,
        )
    }
}
