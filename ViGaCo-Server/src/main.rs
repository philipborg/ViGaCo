extern crate pretty_env_logger;
#[macro_use] extern crate log;

use uinput::Device;
use futures::{StreamExt};
use warp::ws::{Message, WebSocket};
use warp::Filter;
use uinput::device;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::path("controller")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(move |websocket| controller_connected(websocket)));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn controller_connected(ws: WebSocket) {
    info!("New controller request!");
    let device: Device = create_device().await.unwrap();

    let (tx, mut rx) = ws.split();
    
    while let Some(result) = rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("Websocket error: {}", e);
                break;
            }
        };
        process_message(&device, msg).await;
    }
}

async fn process_message(device: &Device, msg: Message){
    
}

async fn create_device() -> Result<Device, uinput::Error> {
    return device::Builder::default()?
    .name("ViGiCo")?
    .event(uinput::event::Controller::All)?
    .create();
}
