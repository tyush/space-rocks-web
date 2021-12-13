use bevy::prelude::*;

const ARENA_WIDTH: u32 = 100;
const ARENA_HEIGHT: u32 = 100;

pub struct WrapScreen;

pub fn wrap_screen(
    mut items: Query<&mut Transform, With<WrapScreen>>,
    windows: Res<Windows>,
    // textures: Res<TextureResources>, TODO: determine when we need to wrap via when the sprite is fully off-screen
) {
    let window = windows.get_primary().unwrap();
    let max_x = window.width() / 2.0;
    let max_y = window.height() / 2.0;
    for mut trans in items.iter_mut() {
        match trans.translation.x {
            x if x > max_x => {
                trans.translation.x = -max_x;
            }
            x if x < -max_x => {
                trans.translation.x = max_x;
            }
            _ => (),
        };
        match trans.translation.y {
            y if y > max_y => {
                trans.translation.y = -max_y;
            }
            y if y < -max_y => {
                trans.translation.y = max_y;
            }
            _ => (),
        };
    }
}
pub struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}
