# Helper file for simple commands


BLADERF_HEADER=/usr/local/include/libbladeRF.h
BLADERF_LIB=/usr/local/lib/libbladeRF.dylib

BINDGEN_ARGS=--no-unstable-rust --blacklist-type "bladerf_stream" \
--whitelist-function "bladerf_.*" --raw-line="pub enum bladerf_stream { }"\
--link=bladeRF --output src/libbladerf.rs

# Tests require a device handle so must be run sequentially
test:
	@RUST_TEST_THREADS=1 cargo test

test-bt:
	@RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test -- --nocapture

build:
	cargo build

run:
	cargo run

generate:	
	bindgen $(BINDGEN_ARGS) $(BLADERF_HEADER)
	# Fix *mut *mut to &*mut, why on earth you would want the former..?
	sed -i '' 's/\*mut \*mut/\&\*mut/' src/libbladerf.rs
	sed -i '' 's/devinfo: \*mut bladerf_devinfo/\devinfo: *const bladerf_devinfo/' src/libbladerf.rs
	# Add link name because lolwat
	sed -i '' 's/^extern "C" {/#[link(name = "bladeRF")] extern "C" {/' src/libbladerf.rs