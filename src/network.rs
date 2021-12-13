use crate::map::RaceMap;
use crate::SERVER_IP_ADDRESS;
use bevy::prelude::*;
use futures::executor::block_on;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn reload();

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn close() {
    #[cfg(not(target_arch = "wasm32"))]
    panic!();
    #[cfg(target_arch = "wasm32")]
    reload();
}

pub async fn get_map_from_server() -> Result<RaceMap, JsValue> {
    log("Making map request...");
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::NoCors);

    let url = SERVER_IP_ADDRESS.to_owned() + "/map.json";

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Use serde to parse the JSON into a struct.
    let map: RaceMap = json.into_serde().unwrap();
    log(&format!("{:?}", map));

    // Send the `Branch` struct back to JS as an `Object`.
    Ok(map)
}

// pub async fn get_map_from_server() -> reqwest::Result<Response> {
//     reqwest::get(SERVER_IP_ADDRESS.to_owned() + "/map.json").await
// }

// pub fn listen_for_map() {
//     let resp = block_on(reqwest::get(SERVER_IP_ADDRESS.to_owned() + "/map.json"));
//     #[cfg(target_arch = "wasm32")]
//     log(&format!("{:?}",resp));
//     match resp {
//         Ok(r) => {
//             let text = block_on(r.text()).unwrap();
//             println!("got map data: {}", text);
//             if text.len() > 10 {
//                 #[cfg(target_arch = "wasm32")]
//                 reload();
//                 #[cfg(not(target_arch = "wasm32"))]
//                 panic!("Awaiting next map...");
//                 unreachable!()
//             }
//         }
//         Err(_) => {
//             #[cfg(target_arch = "wasm32")]
//             reload();
//             #[cfg(not(target_arch = "wasm32"))]
//             panic!("Server is offline or unreachable!");
//             unreachable!()
//         },
//     };
// }

// pub fn load_map_from_network(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     meshes: ResMut<Assets<Mesh>>,
// ) {
//     // block_on allows this function to be effectively sync
//     let resp = block_on(reqwest::get(SERVER_IP_ADDRESS.to_owned() + "/map.json"));
//     println!();

//     match resp {
//         Ok(r) => {
//             let map = block_on(r.json::<RaceMap>());
//             if let Ok(r) = map {
//                 r.put_into_world(
//                     &mut commands,
//                     materials.add(ColorMaterial::color(Color::WHITE)),
//                     meshes,
//                 )
//             }
//         }
//         Err(_) => {
//             #[cfg(target_arch = "wasm32")]
//             reload();
//             #[cfg(not(target_arch = "wasm32"))]
//             panic!("Server is offline or unreachable!");
//             unreachable!()
//         },
//     };
// }

// pub fn load_map_from_network() -> RaceMap {
//     let (resp)
// }
