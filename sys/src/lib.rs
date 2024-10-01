//! Bindgen generated bindings for libbladerf

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl bladerf_devinfo {
    /// Fetch serial string
    pub fn serial(&self) -> String {
        let mut s = [0u8; 33];
        for i in 0..self.serial.len() {
            s[i] = self.serial[i] as u8;
        }

        CStr::from_bytes_until_nul(&s)
            .map(|v| v.to_string_lossy().into_owned())
            .unwrap_or("".to_string())
    }
}

#[cfg(test)]
mod tests {
    // TODO: any useful binding tests?
}
