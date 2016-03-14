# Accordare

a toy project in Rust for my instrument and web application course.

###1. Install
Since this is an application using Rust, Rust implementation is necessary. In order to run this project, Rust should be the nightly version.

Firstly, install rust environment:

	curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly

After installing rust, clone this project and build it with cargo:

	git clone https://github.com/SughiY/Accordare.git && cd ./Accordare/ && cargo build

Finally type `cargo run` in shell command to run this project and then open the index.html in path `./static/index.html`

Or you can just clone this project and then use the script:

	git clone https://github.com/SughiY/Accordare.git && cd ./Accordare/ && ./run.sh

###2. Dependencies
This is a project in [Rust](https://www.rust-lang.org) a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. 

Creation of Web application with [IRON]([http://ironframework.io) An Extensible, Concurrent Web Framework based on middleware [MIO](https://github.com/carllerche/mio) an event loop non-block IO library for Rust.

Since there's no lib like **socket.io** for Iron, [EventEmiiter.js](https://github.com/Olical/EventEmitter) is used with **Websocket** to imitate event-loop api on the browser side. 

Besides, for realizing animation on browser side, we finally choose [snabbt.js](https://daniel-lundin.github.io/snabbt.js) ,a minimalistic javascript animation library, which is very easy to use.

For later object, [Hound](https://github.com/ruud-v-a/hound) ,A wav encoding and decoding library in Rust, and [Yin pitch detection algorithm](https://github.com/ashokfernandez/Yin-Pitch-Tracking) used for detecting pitch in a wav frame.

###3. Architecture
Architecture is separate by two parts : Server side (Iron) + Client side(Firefox).

All the logic can be implemented by rust, but Rust provides a Foreign Function Interface (FFI) to C libraries. So it is also possible to utilise C lib directly on the server side. 

Client side uses webrtc to record audio into .wav format and then sends to server by websocket.

Server side decode .wav file to obtain wave information which will be treated by Yin - alogrithm to get pitch and other infos, etc.