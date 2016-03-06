# Accordare

a toy project in Rust for my instrument and web application course.

###1. Install
Because this is an application using Rust, rust implementation is necessary. For this project, rust should be the nightly version.

So Firstly, install rust environment:

	curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly

After installing rust, clone this project and build it with cargo:

	git clone https://github.com/SughiY/Accordare.git && cd ./Accordare/ && cargo build
	
Finally type `cargo run` in shell command to run this project and then open the index.html in path `./static/index.html`

Or you can clone this project and then run with script:

	git clone https://github.com/SughiY/Accordare.git && cd ./Accordare/ && ./run.sh
