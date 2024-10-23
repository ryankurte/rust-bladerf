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
        for (s_val, ser) in s.iter_mut().zip(self.serial) {
            *s_val = ser as u8;
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
