use crate::map::RaceMap;
use crate::SERVER_IP_ADDRESS;
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

    request.headers().set("Accept", "application/json")?;

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
