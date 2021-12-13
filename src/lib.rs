pub mod input;
pub mod map;
pub mod network;
pub mod objects;
pub mod physics;
pub mod screen;
pub mod ubevy;

const SHIP_TURN_SPEED: f32 = 8.0; // arbitrary, adjust as needed
const SHIP_SPEED: f32 = 600.0; // arbitrary, adjust as needed

/// DON'T CHANGE THIS WITHOUT UPDATING THE ONE IN main.rs:3
const SERVER_IP_ADDRESS: &str = "http://localhost:8000";
