use std::{error::Error, ffi::CStr, fmt::Display};

use bladerf_sys as sys;

/// Error Codes as defined in <https://nuand.com/libbladeRF-doc/v2.5.0/group___r_e_t_c_o_d_e_s.html>
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum BladeRfError {
    Unexpected = -1,
    Range = -2,
    Inval = -3,
    Mem = -4,
    Io = -5,
    Timeout = -6,
    Nodev = -7,
    Unsupported = -8,
    Misaligned = -9,
    Checksum = -10,
    NoFile = -11,
    UpdateFpga = -12,
    UpdateFw = -13,
    TimePast = -14,
    QueueFull = -15,
    FpgaOp = -16,
    Permission = -17,
    WouldBlock = -18,
    NotInit = -19,
    /// Arbitrarily chosen discriminant
    Unknown(i32) = i32::MIN,
}

impl BladeRfError {
    pub fn from_code(code: i32) -> Self {
        match code {
            -1 => Self::Unexpected,
            -2 => Self::Range,
            -3 => Self::Inval,
            -4 => Self::Mem,
            -5 => Self::Io,
            -6 => Self::Timeout,
            -7 => Self::Nodev,
            -8 => Self::Unsupported,
            -9 => Self::Misaligned,
            -10 => Self::Checksum,
            -11 => Self::NoFile,
            -12 => Self::UpdateFpga,
            -13 => Self::UpdateFw,
            -14 => Self::TimePast,
            -15 => Self::QueueFull,
            -16 => Self::FpgaOp,
            -17 => Self::Permission,
            -18 => Self::WouldBlock,
            -19 => Self::NotInit,
            x => Self::Unknown(x),
        }
    }
}

impl From<BladeRfError> for i32 {
    fn from(value: BladeRfError) -> Self {
        match value {
            BladeRfError::Unexpected => -1,
            BladeRfError::Range => -2,
            BladeRfError::Inval => -3,
            BladeRfError::Mem => -4,
            BladeRfError::Io => -5,
            BladeRfError::Timeout => -6,
            BladeRfError::Nodev => -7,
            BladeRfError::Unsupported => -8,
            BladeRfError::Misaligned => -9,
            BladeRfError::Checksum => -10,
            BladeRfError::NoFile => -11,
            BladeRfError::UpdateFpga => -12,
            BladeRfError::UpdateFw => -13,
            BladeRfError::TimePast => -14,
            BladeRfError::QueueFull => -15,
            BladeRfError::FpgaOp => -16,
            BladeRfError::Permission => -17,
            BladeRfError::WouldBlock => -18,
            BladeRfError::NotInit => -19,
            BladeRfError::Unknown(x) => x,
        }
    }
}

impl Display for BladeRfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disciminant = i32::from(*self);
        // Safety: the function https://github.com/Nuand/bladeRF/blob/fe3304d75967c88ab4f17ff37cb5daf8ff53d3e1/host/libraries/libbladeRF/src/bladerf.c#L1784
        // Returns a valid Cstring for any i32 input.
        let msg_ptr = unsafe { sys::bladerf_strerror(disciminant) };
        let msg = unsafe {
            CStr::from_ptr(msg_ptr)
                .to_str()
                .expect("These strings all seems to be valid ascii and thus UTF8")
        };
        write!(f, "BladeRF Error: {}", msg)
    }
}

impl Error for BladeRfError {}
