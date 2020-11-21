/***
 * 
 * 
 * 
    RELEASE BUILD FOR A WASM TARGET

    cargo build --target wasm32-unknown-unknown --release

    By default, Cargo puts the build artifacts in target/<target-triple>/<mode>/
    or target/<mode> for the default target which in this case is
    target/wasm32-unknown-unknown/release

    check out the file size after the build: ls -lh target/wasm32-unknown-unknown/release/do_nothing.wasm
***/