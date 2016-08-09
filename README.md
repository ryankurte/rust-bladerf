# Rust-BladeRF

## Status

Very much a WIP. Most methods are implemented, API expected to change a whole lot, and needs a lot of cleaning up.

## Requirements

To run you need to have the appropriate FPGA .rbf file in your working directory, otherwise the FPGA will not be automatically loaded.

## Testing

Run tests with `make test`.  
This is required because cargo automagically uses multiple test threads, which doesn't work when each test requires it's own bladeRF.
