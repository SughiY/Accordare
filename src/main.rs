/// Simple WebSocket server with error handling. It is not necessary to setup logging, but doing
/// so will allow you to see more details about the connection by using the RUST_LOG env variable.
extern crate ws;
extern crate env_logger;
extern crate rustc_serialize;
extern crate libc;

use ws::listen;
use ws::Message;
use rustc_serialize::json;

mod Acc;

#[derive(RustcDecodable, RustcEncodable)]
struct EventObject{
    event:String,
    content:f32
}

fn main () {

    // Setup logging
    env_logger::init().unwrap();
    // Listen on an address and call the closure for each connection
    if let Err(error) = listen("127.0.0.1:3012", |out| {
        // The handler needs to take ownership of out, so we use move
        move |msg:Message| {
            let result = Acc::estimate_pitch(msg.into_data());
            let object = EventObject{
                event: "pitch".to_string(),
                content: result.get_pitch()
            };

            if let Ok(json_file) = json::encode(&object) {
                out.send(json_file)   
            } else {
                Ok(())
            }
        }
    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to {:?}", error);
    }
}
