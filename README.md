# Rust-BladeRF

## Status

You can connect to the device, read the version, and disconnect from it.  
The bindings are fairly complete, but the wrappers are missing all of the important functionality (though PRs are welcome!).  

## Testing

Run tests with `RUST_TEST_THREADS=1 cargo test -- --nocapture`.  
This is required because cargo automagically uses multiple test threads, which doesn't work when each test requires it's own bladeRF.
