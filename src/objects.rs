use crate::{input::UserInputs, map::TrackRail, network::close, StaysNearShip};

use bevy::{prelude::*, sprite::collide_aabb::collide};

pub struct Ship {
    pub inputs: Vec<UserInputs>,
}

impl Default for Ship {
    fn default() -> Self {
        Self { inputs: Vec::new() }
    }
}

pub struct Timer(pub f32);

impl Timer {
    pub fn new(
        // font: Handle<Font>,
        x: f32,
        y: f32,
        initial: f32,
        handle: Handle<Font>,
        commands: &mut Commands,
    ) {
        commands
            .spawn_bundle(Text2dBundle {
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    {
                        if initial >= 10.0 {
                            format!("{:0.5}", initial.to_string())
                        } else if initial < 10.0 {
                            format!("0{:0.4}", initial.to_string())
                        } else {
                            initial.to_string()
                        }
                    },
                    TextStyle {
                        font_size: 100.0,
                        color: Color::WHITE,
                        font: handle,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            })
            .insert(Timer(0.0))
            .insert(Transform::from_xyz(x, y, 100.0))
            .insert(StaysNearShip(0.0, 300.0));
    }
}

pub fn timer_system(
    time: Res<Time>,
    mut timers: Query<(&mut Timer, &mut Text)>,
    asset_server: Res<AssetServer>,
) {
    for (mut time_left, mut text) in timers.iter_mut() {
        time_left.0 += time.delta_seconds();
        if time_left.0 >= 10.0 {
            text.sections[0].value = format!("{:0.5}", time_left.0.to_string());
        } else if time_left.0 <= 10.0 && time_left.0 >= 0.0 {
            text.sections[0].value = format!("0{:0.4}", time_left.0.to_string());
        } else {
            text.sections[0].style.color = Color::RED;
            text.sections[0].style.font = asset_server.load("arial_bold.ttf");
            if time_left.0 as i32 % 2 != 0 {
                text.sections[0].value = "00.00".to_string();
            } else {
                text.sections[0].value = "     ".to_string();
            }
        }
    }
}

pub fn did_ship_die(rails: Query<(&Transform, &TrackRail)>, ship: Query<&Transform, With<Ship>>) {
    let ship_size = Vec2::new(24.0, 24.0);
    let ship = ship.single();
    if let Ok(ship) = ship {
        for rail in rails.iter() {
            // if collide(
            //     ship.translation,
            //     ship_size,
            //     Vec3::new(
            //         rail.0.translation.x,
            //         rail.0.translation.y,
            //         rail.0.translation.z,
            //     ),
            //     Vec2::new(rail.1.length.powi(2), rail.1.length.powi(2)),
            // )
            // .is_some()
            // MASSIVE FUCKING TODO: ONLY CHECK COLLISION IS SHIP IS NEAR!
            {
                // info!("Ship nearing collision!");
                for point in rail.1.subpoints.iter() {
                    // info!("Checking linepoint {:?}", point);
                    if collide(
                        ship.translation,
                        ship_size,
                        Vec3::new(point.0, point.1, 0.0),
                        Vec2::new(6.0, 6.0),
                    )
                    .is_some()
                    {
                        close();
                    }
                }
            }
        }
    }
}
