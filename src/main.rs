use std::task::Context;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use space_rocks_web::{
    input::{input_parser, movements_to_vel},
    map::{map_from_ron, RaceMap},
    network::{close, get_map_from_server},
    objects::{timer_system, Ship},
    physics::{
        angular::{angular_momentum, AngularMomentum},
        velocity::{velocity, Velocity},
    },
};
use wasm_bindgen_futures::spawn_local;

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen;

#[derive(StageLabel, SystemLabel, Clone, Copy, Hash, Debug, PartialEq, Eq)]
enum Stages {
    InputHandling,
    Physics,
    ScreenAdjustment,
}

fn main() {
    create_test_game();
    spawn_local(run_app())
}

async fn run_app() {
    println!("Hello, world!");
    // create_test_game();

    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    // #[cfg(target_arch = "wasm32")]
    // wasm_bindgen::web_console::log_1(&"Starting game creation...".into());

    // let mut map = get_map_from_server();

    // async move {
    //     map.then(|m| {
    //         m.unwrap().json::<RaceMap>().then(|race_map| {
    //             let race_map = race_map.unwrap();

    //             create_app(race_map);
    //         });
    //     });
    // }

    // get_map_from_server().then(|map| {
    //     if let Ok(m) = map {
    //         println!("Received map {:?}", &m);
    //         create_app(m);
    //     } else {
    //         panic!("Could not load map from server!")
    //     }
    //     get_map_from_server()
    // });

    let g = get_map_from_server().await;

    match g {
        Ok(map) => create_app(&map),
        Err(_e) => panic!("Unable to parse json map from server!"),
    }

    // g.inspect(|map| {
    //     if let Ok(m) = map {
    //         println!("Received map {:?}", &m);
    //         create_app(m);
    //     } else {
    //         close();
    //         panic!("Could not load map from server!")
    //     }
    // });
}

fn create_app(map: &RaceMap) {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.insert_resource(map.clone_me());

    app.add_startup_system(setup.system())
        .add_system(input_parser.system().label(Stages::InputHandling))
        .add_system(
            movements_to_vel
                .system()
                .label(Stages::Physics)
                .after(Stages::InputHandling),
        )
        .add_system(
            velocity
                .system()
                .after(Stages::InputHandling)
                .label(Stages::Physics),
        )
        .add_system(
            angular_momentum
                .system()
                .label(Stages::Physics)
                .after(Stages::InputHandling),
        )
        .add_system(
            follows_ship
                .system()
                .after(Stages::Physics)
                .label(Stages::ScreenAdjustment),
        )
        .add_system(timer_system.system())
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .run();
}

fn create_test_game() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_startup_system(setup.system())
        .add_startup_system(add_test_track.system())
        .add_system(input_parser.system().label(Stages::InputHandling))
        .add_system(
            movements_to_vel
                .system()
                .label(Stages::Physics)
                .after(Stages::InputHandling),
        )
        .add_system(
            velocity
                .system()
                .after(Stages::InputHandling)
                .label(Stages::Physics),
        )
        .add_system(
            angular_momentum
                .system()
                .label(Stages::Physics)
                .after(Stages::InputHandling),
        )
        .add_system(
            follows_ship
                .system()
                .after(Stages::Physics)
                .label(Stages::ScreenAdjustment),
        )
        .add_system(timer_system.system())
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    map: Option<Res<RaceMap>>,
) {
    // let bevy handle changed assets in real time
    // but only if running on native
    // #[cfg(not(target_arch = "wasm32"))]
    asset_server.watch_for_changes().unwrap();

    // make it look like i put effort into this
    let primary_window = windows.get_primary_mut().unwrap();

    // don't hide cursor if we are in dev build so we can use inspector
    #[cfg(debug_assertions)]
    // primary_window.set_cursor_visibility(false);

    // primary_window.set_resizable(false);
    // primary_window.set_scale_factor_override(Some(1.0));
    #[cfg(target_arch = "wasm32")]
    primary_window.set_resolution(1280.0, 720.0);
    #[cfg(not(target_arch = "wasm32"))]
    primary_window.set_resolution(200.0, 200.0);

    primary_window
        .set_title("Entirely Too Many Rocks, Just Seriously, There Are Too Many.".to_owned());

    let ship_handle: Handle<Texture> =
        asset_server.load("ship.png");

    // commands.insert_resource(TextureResources {
    //     ship: materials.add((&ship_handle).into()),
    // });

    let ship_handle = materials.add(ship_handle.into());

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(FollowsShip);

    // ship "model"
    // let ship_model = shape::;
    // ship
    commands
        .spawn_bundle(SpriteBundle {
            material: ship_handle,
            ..Default::default()
        })
        .insert(Ship::default())
        // .insert(WrapScreen)
        // .insert(screen::Size::square(2.0))
        .insert(Velocity(0.0, 0.0, 0.4))
        .insert(AngularMomentum(0.0, 0.8))
        // .insert(ColliderBundle {
        //     shape: ColliderShape::round_cuboid(1.0, 1.0, 0.3),
        //     collider_type: ColliderType::Solid,
        //     material: ColliderMaterial { friction: 0.7, ..Default::default() },
        //     ..Default::default()
        // })
        // .insert(RigidBodyBundle {
        //     body_type: RigidBodyType::KinematicVelocityBased,
        //     position: Vec2::new(30.0, 30.0).into(),
        //     forces: RigidBodyForces { gravity_scale: 0.0, ..Default::default() },
        //     activation: RigidBodyActivation::cannot_sleep(),
        //     ccd: RigidBodyCcd { ccd_enabled: true, ..Default::default() },
        //     ..Default::default()
        // })
        ;

    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: materials.add(asset_server.load("other_ship.png").into()),
    //         ..Default::default()
    //     })
    //     .insert(Ship::default())
    //     .insert(WrapScreen)
    //     .insert(screen::Size::square(2.0))
    //     .insert(Velocity(0.0, 20.0, 0.2))
    //     .insert(AngularMomentum(0.0, 0.8));

    // commands
    //     .spawn_bundle(Timer::new(asset_server.load("arial.ttf"), 20.0));
    let handle = materials.add(ColorMaterial::color(Color::WHITE));
    if let Some(m) = map {
        m.put_into_world(&mut commands, handle, meshes);
    }
}

fn add_test_track(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let m = map_from_ron(
        "
    RaceMap(
        track_points: [
            (00.0, 00.0),
            (220.0, 30.0),
            (440.0, -400.0),
            (400.0, -800.0),
            (220.0, -700.0)
        ],
        track_points_other: [
            (40.0, 40.0),
            (260.0, 70.0),
            (480.0, -360.0),
            (440.0, -760.0),
            (260.0, -660.0)
        ],
        finish_line: (
            (30.0, 20.0),
            (30.0, 30.0)
        )   
    )
    ",
    );

    let handle = materials.add(ColorMaterial::color(Color::WHITE));

    m.unwrap().put_into_world(&mut commands, handle, meshes);
}

struct FollowsShip;

fn follows_ship(
    mut query: QuerySet<(
        Query<&mut Transform, With<FollowsShip>>,
        Query<&Transform, With<Ship>>,
    )>,
) {
    let ship_pos = query.q1().single().unwrap().translation;

    for mut trans in query.q0_mut().iter_mut() {
        trans.translation = Vec3::new(
            ship_pos.x,
            ship_pos.y,
            trans.translation.z, // maintain previous visibility
        );
    }
}

// fn listen_for_map(
//     map_res: ResMut<RaceMapHolder>
// ) {

// }
