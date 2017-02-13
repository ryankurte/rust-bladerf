
use std::*;
#[allow(dead_code, non_camel_case_types)]

mod libbladerf;
use libbladerf::*;

impl ::std::clone::Clone for Struct_bladerf_devinfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_bladerf_devinfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
