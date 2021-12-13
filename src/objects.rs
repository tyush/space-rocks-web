use crate::input::UserInputs;

use bevy::prelude::*;

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
    pub fn new(font: Handle<Font>, initial: f32) -> (bevy::prelude::Text2dBundle, Timer, bevy::prelude::Transform) {
        (Text2dBundle {
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
                    font
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        },
        Timer(12.0),
        Transform::from_xyz(1.0, 1.0, 100.0))
    }
}

pub fn timer_system(time: Res<Time>, mut timers: Query<(&mut Timer, &mut Text)>, asset_server: Res<AssetServer>) {
    for (mut time_left, mut text) in timers.iter_mut() {
        time_left.0 -= time.delta_seconds();
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