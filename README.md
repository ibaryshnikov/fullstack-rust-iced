# Fullstack Rust example with iced and axum

An example of building a crossplatform application with iced,
covering desktop and web targets. Combined with axum server, it
provides a boilerplate for fullstack development in Rust.


## Usage

```bash
# Start server
cd server
cargo run

# Start desktop app
cd desktop
cargo run

# Build web client
# First, install wasm-bindgen-cli which should match the version in Cargo.toml
cargo install wasm-bindgen-cli --version 0.2.105
# Then build the client for WebAssembly target
cd web
./build.sh
# then navigate to http://127.0.0.1:8080/client
