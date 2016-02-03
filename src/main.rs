/// Simple WebSocket server with error handling. It is not necessary to setup logging, but doing
/// so will allow you to see more details about the connection by using the RUST_LOG env variable.
extern crate ws;
extern crate env_logger;
extern crate rustc_serialize;

use ws::listen;
use ws::Message;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct EventObject{
    event:String,
    content:String
}

fn main () {

    // Setup logging
    env_logger::init().unwrap();
    // Listen on an address and call the closure for each connection
    if let Err(error) = listen("127.0.0.1:3012", |out| {
        // The handler needs to take ownership of out, so we use move
        move |msg:Message| {
            // Handle messages received on this connection
//            let text = msg.clone().into_text().unwrap();
//            let json_object = json::decode(&text).unwrap();
//            println!("Server got message '{}'. ", json_object);
            // Use the out channel to send messages back
            let ob = EventObject{ event : "foo".to_string(),
                             content: "just for test".to_string() };
            let text = json::encode(&ob).unwrap(); 
            out.send(text)
        }
    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to {:?}", error);
    }
}
