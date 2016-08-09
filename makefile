
test:
	@RUST_TEST_THREADS=1 cargo test -- --nocapture

test-bt:
	@RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test -- --nocapture
