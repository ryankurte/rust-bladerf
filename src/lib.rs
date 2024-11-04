//! Safe bindings for libbladerf (wrapping bladerf-sys)
//!
//!
#![allow(non_upper_case_globals)]

use std::mem::*;
use std::*;

use cmp::Ordering;
use num_complex::Complex;

use bladerf_sys::*;

pub mod error;

// Macro to simplify integer returns
macro_rules! handle_res {
    ($e:expr) => (
    	if $e >= 0 {
			return Ok($e as isize)
		} else {
			return Err($e as isize)
		}
	);
	($res:expr, $out:expr) => (
		if $res >= 0 {
			return Ok($out)
		} else {
			return Err($res as isize)
		}
	);
}

// BladeRF module config object
#[derive(Clone, Debug)]
pub struct BladeRFModuleConfig {
    pub frequency: u64,
    pub sample_rate: u32,
    pub bandwidth: u32,
    pub lna_gain: i32,
    pub vga1: i32,
    pub vga2: i32,
}

// BladeRF overall config object
pub struct BladeRFConfig {
    pub tx: BladeRFModuleConfig,
    pub rx: BladeRFModuleConfig,
}

// BladeRF device object
pub struct BladeRF {
    device: *mut bladerf,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum BladeRFChannel {
    Rx1 = bladerf_channel_layout_BLADERF_RX_X1,
    Rx2 = bladerf_channel_layout_BLADERF_RX_X2,
    Tx1 = bladerf_channel_layout_BLADERF_TX_X1,
    Tx2 = bladerf_channel_layout_BLADERF_TX_X2,
}

/// Loopback configuration
///
/// wraps bladerf_loopback
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum BladeRFLoopback {
    None = bladerf_loopback_BLADERF_LB_NONE,
    RfLna1 = bladerf_loopback_BLADERF_LB_RF_LNA1,
    RfLna2 = bladerf_loopback_BLADERF_LB_RF_LNA2,
    RfLna3 = bladerf_loopback_BLADERF_LB_RF_LNA3,
    Firmware = bladerf_loopback_BLADERF_LB_FIRMWARE,
    RficBist = bladerf_loopback_BLADERF_LB_RFIC_BIST,
    BbTxlpfRxlpf = bladerf_loopback_BLADERF_LB_BB_TXLPF_RXLPF,
    BbTxlpfRxvga2 = bladerf_loopback_BLADERF_LB_BB_TXLPF_RXVGA2,
    BbTxvga1Rxlpf = bladerf_loopback_BLADERF_LB_BB_TXVGA1_RXLPF,
    BbTxvga1Rxvga2 = bladerf_loopback_BLADERF_LB_BB_TXVGA1_RXVGA2,
}

impl TryFrom<bladerf_loopback> for BladeRFLoopback {
    type Error = bladerf_loopback;

    fn try_from(value: bladerf_loopback) -> Result<Self, bladerf_loopback> {
        let v = match value {
            bladerf_loopback_BLADERF_LB_NONE => Self::None,
            bladerf_loopback_BLADERF_LB_RF_LNA1 => Self::RfLna1,
            bladerf_loopback_BLADERF_LB_RF_LNA2 => Self::RfLna2,
            bladerf_loopback_BLADERF_LB_RF_LNA3 => Self::RfLna3,
            bladerf_loopback_BLADERF_LB_FIRMWARE => Self::Firmware,
            bladerf_loopback_BLADERF_LB_RFIC_BIST => Self::RficBist,
            bladerf_loopback_BLADERF_LB_BB_TXLPF_RXLPF => Self::BbTxlpfRxlpf,
            bladerf_loopback_BLADERF_LB_BB_TXLPF_RXVGA2 => Self::BbTxlpfRxvga2,
            bladerf_loopback_BLADERF_LB_BB_TXVGA1_RXLPF => Self::BbTxvga1Rxlpf,
            bladerf_loopback_BLADERF_LB_BB_TXVGA1_RXVGA2 => Self::BbTxvga1Rxvga2,
            _ => return Err(value),
        };

        Ok(v)
    }
}

impl Drop for BladeRF {
    fn drop(&mut self) {
        // Safety: the open functions will initialize self.device
        //    and make it null or a valid pointer.
        unsafe { bladerf_close(self.device) }
    }
}

pub fn set_usb_reset_on_open(enabled: bool) {
    unsafe {
        bladerf_set_usb_reset_on_open(enabled);
    }
}

impl BladeRF {
    /// List attached BladeRF devices
    pub fn get_device_list() -> Result<Vec<bladerf_devinfo>, isize> {
        let mut devices = MaybeUninit::<*mut bladerf_devinfo>::uninit();

        // Safety: This function is responsible for initializing the devices pointer
        // It will return n>0 if initialized.
        // https://github.com/Nuand/bladeRF/blob/fe3304d75967c88ab4f17ff37cb5daf8ff53d3e1/host/libraries/libbladeRF/src/devinfo.c#L58
        // Does this memory only need to be freed if it is sucessfully initialized?
        let n = unsafe { bladerf_get_device_list(devices.as_mut_ptr()) as isize };

        // Catch bladerf function errors
        if n > 0 {
            // Cast array to slice and create a safe array to return
            // Safety: Should have a valid pointer of length n. n is <0 if the pointer is invalid.
            // What about n=0?
            // https://github.com/Nuand/bladeRF/blob/fe3304d75967c88ab4f17ff37cb5daf8ff53d3e1/host/libraries/libbladeRF/src/devinfo.c#L58
            let device_slice = unsafe { std::slice::from_raw_parts(devices.as_ptr(), n as usize) };
            let mut safe_device_list: Vec<bladerf_devinfo> = Vec::new();

            for info_ptr in device_slice.iter().copied() {
                // Safety: This should be a valid pointer that lives untill we free the device list.
                // We are copying it into a vector.
                safe_device_list.push(unsafe { *info_ptr });
            }
            unsafe { bladerf_free_device_list(*devices.as_ptr()) };

            // Return rust save device info array
            Ok(safe_device_list)
        } else {
            // Return error code
            Err(n)
        }
    }

    /// Open a BladeRF device by identifier
    pub fn open(identifier: Option<String>) -> Result<Self, isize> {
        let mut bladerf_device = Self {
            device: ptr::null_mut(),
        };

        // Safety: This function is responsible for initializing the device pointer.
        // https://github.com/Nuand/bladeRF/blob/fe3304d75967c88ab4f17ff37cb5daf8ff53d3e1/host/libraries/libbladeRF/src/bladerf.c#L94
        // It will either assign it null or a valid pointer
        let res = match identifier {
            Some(id) => {
                let c_string = ffi::CString::new(id).unwrap();
                unsafe { bladerf_open(&mut bladerf_device.device, c_string.as_ptr()) }
            }
            None => unsafe { bladerf_open(&mut bladerf_device.device, ptr::null()) },
        };

        handle_res!(res, bladerf_device);
    }

    /// Open a BladeRF device by devinfo object
    pub fn open_with_devinfo(mut devinfo: bladerf_devinfo) -> Result<Self, isize> {
        let devinfo_ptr: *mut bladerf_devinfo = &mut devinfo as *mut bladerf_devinfo;

        let mut bladerf_device = Self {
            device: ptr::null_mut(),
        };

        // Safety: This function is responsible for initializing the device pointer.
        // https://github.com/Nuand/bladeRF/blob/fe3304d75967c88ab4f17ff37cb5daf8ff53d3e1/host/libraries/libbladeRF/src/bladerf.c#L110
        // It will either assign it null or a valid pointer
        let res = unsafe { bladerf_open_with_devinfo(&mut bladerf_device.device, devinfo_ptr) };

        handle_res!(res, bladerf_device);
    }

    // Device Properties and Information
    // http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___i_n_f_o.html

    /// Gets the serial number of the BladeRF
    pub fn get_serial(&self) -> Result<String, isize> {
        // Create raw data array for serial return
        let mut serial_data: Vec<::libc::c_char> = vec![0; 33];

        // Call underlying c method
        // Safety: This method will be called on a pointer that is initialized with a valid pointer (wont be null either)
        // We should use a type to better enforce this.
        // serial_data is expected to be a slice of length 33.
        // This function is depricated, in favor of bladerf_get_serial_struct()?
        let res = unsafe { bladerf_get_serial(self.device, serial_data.as_mut_ptr()) };

        if res >= 0 {
            // Map ::libc::c_char back to u8 as required for string manipulation
            let serial_u8: Vec<u8> = serial_data.iter().map(|&x| x as u8).collect();

            // Build String
            // Safety: it is unclear if if the vector will actually be null terminated
            //   it is initialized it with zeros,
            //   so I presume that this function will only write up to 32 characters and not touch the last value.
            let serial_cstr = unsafe { ffi::CString::from_vec_unchecked(serial_u8) };
            let serial_str = serial_cstr.into_string().unwrap();

            Ok(serial_str)
        } else {
            Err(res as isize)
        }
    }

    pub fn get_fpga_size(&self) -> Result<bladerf_fpga_size, isize> {
        let mut fpga_size: bladerf_fpga_size = bladerf_fpga_size_BLADERF_FPGA_UNKNOWN;

        let res = unsafe { bladerf_get_fpga_size(self.device, &mut fpga_size) };

        handle_res!(res, fpga_size);
    }

    pub fn fw_version(&self) -> Result<bladerf_version, isize> {
        let mut version = bladerf_version {
            major: 0,
            minor: 0,
            patch: 0,
            describe: std::ptr::null::<i8>(),
        };

        let res = unsafe { bladerf_fw_version(self.device, &mut version) };

        handle_res!(res, version);
    }

    pub fn is_fpga_configured(&self) -> Result<bool, isize> {
        let res = unsafe { bladerf_is_fpga_configured(self.device) };

        match res.cmp(&0) {
            Ordering::Greater => Ok(true),
            Ordering::Equal => Ok(false),
            Ordering::Less => Err(res as isize),
        }
    }

    pub fn fpga_version(&self) -> Result<bladerf_version, isize> {
        let mut version = bladerf_version {
            major: 0,
            minor: 0,
            patch: 0,
            describe: std::ptr::null::<i8>(),
        };

        let res = unsafe { bladerf_fpga_version(self.device, &mut version) };

        handle_res!(res, version);
    }

    // RX & TX Module Control
    // http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___m_o_d_u_l_e.html

    pub fn enable_module(&self, module: bladerf_module, enable: bool) -> Result<isize, isize> {
        let res = unsafe { bladerf_enable_module(self.device, module, enable) };

        handle_res!(res);
    }

    // Gain Control
    // http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___g_a_i_n.html

    pub fn set_lna_gain(&self, gain: bladerf_lna_gain) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_lna_gain(self.device, gain) };

        handle_res!(res);
    }

    pub fn get_lna_gain(&self) -> Result<bladerf_lna_gain, isize> {
        let mut gain: bladerf_lna_gain = bladerf_lna_gain_BLADERF_LNA_GAIN_UNKNOWN;

        let res = unsafe { bladerf_get_lna_gain(self.device, &mut gain) };

        handle_res!(res, gain);
    }

    pub fn set_rxvga1(&self, gain: i32) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_rxvga1(self.device, gain) };

        handle_res!(res);
    }

    pub fn get_rxvga1(&self) -> Result<i32, isize> {
        let mut gain: i32 = 0;

        let res = unsafe { bladerf_get_rxvga1(self.device, &mut gain) };

        handle_res!(res, gain);
    }

    pub fn set_rxvga2(&self, gain: i32) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_rxvga2(self.device, gain) };

        handle_res!(res);
    }

    pub fn get_rxvga2(&self) -> Result<i32, isize> {
        let mut gain: i32 = 0;

        let res = unsafe { bladerf_get_rxvga2(self.device, &mut gain) };

        handle_res!(res, gain);
    }

    pub fn set_txvga1(&self, gain: i32) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_txvga1(self.device, gain) };

        handle_res!(res);
    }

    pub fn get_txvga1(&self) -> Result<i32, isize> {
        let mut gain: i32 = 0;

        let res = unsafe { bladerf_get_txvga1(self.device, &mut gain) };

        handle_res!(res, gain);
    }

    pub fn set_txvga2(&self, gain: i32) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_txvga2(self.device, gain) };

        handle_res!(res);
    }

    pub fn get_txvga2(&self) -> Result<i32, isize> {
        let mut gain: i32 = 0;

        let res = unsafe { bladerf_get_txvga2(self.device, &mut gain) };

        handle_res!(res, gain);
    }

    pub fn set_gain(&self, module: bladerf_module, gain: i32) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_gain(self.device, module, gain) };

        handle_res!(res);
    }

    // Sampling Control

    pub fn set_sample_rate(&self, module: bladerf_module, rate: u32) -> Result<u32, isize> {
        let mut actual: u32 = 0;

        let res = unsafe { bladerf_set_sample_rate(self.device, module, rate, &mut actual) };

        handle_res!(res, actual);
    }

    pub fn set_rational_sample_rate(
        &self,
        module: bladerf_module,
        rate: bladerf_rational_rate,
    ) -> Result<bladerf_rational_rate, isize> {
        let mut rate = rate;

        let mut actual = bladerf_rational_rate {
            integer: 0,
            num: 0,
            den: 0,
        };

        let res = unsafe {
            bladerf_set_rational_sample_rate(self.device, module, &mut rate, &mut actual)
        };
        handle_res!(res, actual);
    }

    pub fn get_sample_rate(&self, module: bladerf_module) -> Result<u32, isize> {
        let mut rate: u32 = 0;

        let res = unsafe { bladerf_get_sample_rate(self.device, module, &mut rate) };

        handle_res!(res, rate);
    }

    pub fn get_rational_sample_rate(
        &self,
        module: bladerf_module,
    ) -> Result<bladerf_rational_rate, isize> {
        let mut rate = bladerf_rational_rate {
            integer: 0,
            num: 0,
            den: 0,
        };

        let res = unsafe { bladerf_get_rational_sample_rate(self.device, module, &mut rate) };

        handle_res!(res, rate);
    }

    pub fn set_sampling(&self, sampling: bladerf_sampling) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_sampling(self.device, sampling) };

        handle_res!(res);
    }

    /// Configure RX mux
    pub fn set_rx_mux(&self, mux: bladerf_rx_mux) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_rx_mux(self.device, mux) };

        handle_res!(res);
    }

    /// Fetch RX mux information
    pub fn get_rx_mux(&self) -> Result<bladerf_rx_mux, isize> {
        let mut mux: bladerf_rx_mux = 0;

        let res = unsafe { bladerf_get_rx_mux(self.device, &mut mux as *mut bladerf_rx_mux) };

        handle_res!(res, mux);
    }

    /// Fetch sampling rate
    pub fn get_sampling(&self) -> Result<bladerf_sampling, isize> {
        let mut sampling = bladerf_sampling_BLADERF_SAMPLING_UNKNOWN;

        let res = unsafe { bladerf_get_sampling(self.device, &mut sampling) };

        handle_res!(res, sampling);
    }

    /// Configure bandwidth
    ///
    /// See: http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___b_a_n_d_w_i_d_t_h.html
    pub fn set_bandwidth(&self, module: bladerf_module, bandwidth: u32) -> Result<u32, isize> {
        let mut actual: u32 = 0;

        let res = unsafe { bladerf_set_bandwidth(self.device, module, bandwidth, &mut actual) };

        handle_res!(res, actual);
    }

    /// Fetch bandwidth information
    pub fn get_bandwidth(&self, module: bladerf_module) -> Result<u32, isize> {
        let mut bandwidth: u32 = 0;

        let res = unsafe { bladerf_get_bandwidth(self.device, module, &mut bandwidth) };

        handle_res!(res, bandwidth);
    }

    pub fn set_lpf_mode(
        &self,
        module: bladerf_module,
        lpf_mode: bladerf_lpf_mode,
    ) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_lpf_mode(self.device, module, lpf_mode) };

        handle_res!(res);
    }

    pub fn get_lpf_mode(&self, module: bladerf_module) -> Result<bladerf_lpf_mode, isize> {
        let mut lpf_mode = bladerf_lpf_mode_BLADERF_LPF_NORMAL;

        let res = unsafe { bladerf_get_lpf_mode(self.device, module, &mut lpf_mode) };

        handle_res!(res, lpf_mode);
    }

    //bladerf_set_bandwidth (struct bladerf *dev, bladerf_module module, unsigned int bandwidth, unsigned int *actual)
    //bladerf_get_bandwidth (struct bladerf *dev, bladerf_module module, unsigned int *bandwidth)
    //bladerf_set_lpf_mode (struct bladerf *dev, bladerf_module module, bladerf_lpf_mode mode)
    //bladerf_get_lpf_mode (struct bladerf *dev, bladerf_module module, bladerf_lpf_mode *mode)

    /// Set frequency band
    ///
    /// See: http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___t_u_n_i_n_g.html
    pub fn select_band(&self, module: bladerf_module, frequency: u64) -> Result<isize, isize> {
        let res = unsafe { bladerf_select_band(self.device, module, frequency) };

        handle_res!(res);
    }

    /// Set frequency
    ///
    /// See: http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___t_u_n_i_n_g.html
    pub fn set_frequency(&self, channel: BladeRFChannel, frequency: u64) -> Result<isize, isize> {
        let res =
            unsafe { bladerf_set_frequency(self.device, channel as bladerf_channel, frequency) };

        handle_res!(res);
    }

    /// Fetch frequyency
    pub fn get_frequency(&self, channel: BladeRFChannel) -> Result<u64, isize> {
        let mut freq: u64 = 0;

        let res =
            unsafe { bladerf_get_frequency(self.device, channel as bladerf_channel, &mut freq) };

        handle_res!(res, freq);
    }

    /// Schedule retuning
    pub fn schedule_retune(
        &self,
        module: bladerf_module,
        time: u64,
        frequency: u64,
        quick_tune: Option<bladerf_quick_tune>,
    ) -> Result<isize, isize> {
        let mut quick_tune_int: bladerf_quick_tune;
        let p: *mut bladerf_quick_tune;

        // Check whether quick tune exists and map pointer as appropriate
        match quick_tune {
            Some(qt) => {
                quick_tune_int = qt;
                p = &mut quick_tune_int;
            }
            None => {
                p = ptr::null_mut();
            }
        }

        // Call underlying function
        let res = unsafe { bladerf_schedule_retune(self.device, module, time, frequency, p) };

        // Process response
        handle_res!(res)
    }

    pub fn cancel_scheduled_retune(&self, module: bladerf_module) -> Result<isize, isize> {
        let res = unsafe { bladerf_cancel_scheduled_retunes(self.device, module) } as isize;

        handle_res!(res);
    }

    #[cfg(feature = "unimplemented")]
    pub fn get_quick_tune(&self, module: BladeRFChannel) -> Result<bladerf_quick_tune, isize> {
        let mut quick_tune = bladerf_quick_tune {
            freqsel: 0,
            vcocap: 0,
            nint: 0,
            nfrac: 0,
            flags: 0,
        };

        let res = unsafe { bladerf_get_quick_tune(self.device, module, &mut quick_tune) };

        handle_res!(res, quick_tune);
    }

    pub fn set_tuning_mode(&self, mode: bladerf_tuning_mode) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_tuning_mode(self.device, mode) } as isize;

        handle_res!(res);
    }

    /// Set internal loopback state
    ///
    /// See: http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___l_o_o_p_b_a_c_k.html
    pub fn set_loopback(&self, loopback: BladeRFLoopback) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_loopback(self.device, loopback as bladerf_loopback) };

        handle_res!(res);
    }

    /// Fetch loopback state
    pub fn get_loopback(&self) -> Result<BladeRFLoopback, isize> {
        let mut loopback = bladerf_loopback_BLADERF_LB_NONE;

        let res = unsafe { bladerf_get_loopback(self.device, &mut loopback) };
        if res < 0 {
            return Err(res as isize);
        }

        match BladeRFLoopback::try_from(loopback) {
            Ok(v) => Ok(v),
            Err(_) => Err(-1),
        }
    }

    // SMB Clock Port Control

    // Triggers and Synchronisation

    // Corrections and Calibration

    // Corrections and calibration

    // Expansion boards

    // Expansion IO control

    // Miscellaneous

    // Sample formats and metadata

    // Asynchronous data transmission and reception

    // Synchronous data transmission and reception

    pub fn sync_config(
        &self,
        layout: bladerf_channel_layout,
        format: bladerf_format,
        num_buffers: u32,
        buffer_size: u32,
        num_transfers: Option<u32>,
        stream_timeout: u32,
    ) -> Result<isize, isize> {
        let num_transfers = num_transfers.unwrap_or(4);

        let res = unsafe {
            bladerf_sync_config(
                self.device,
                layout,
                format,
                num_buffers,
                buffer_size,
                num_transfers,
                stream_timeout,
            )
        };

        handle_res!(res);
    }

    pub fn get_timestamp(&self, dir: bladerf_direction) -> u64 {
        let mut value = 0u64;
        unsafe { bladerf_get_timestamp(self.device, dir, &mut value as *mut u64) };

        value
    }

    pub fn sync_tx_meta(
        &self,
        data: &[Complex<i16>],
        meta: &mut bladerf_metadata,
        stream_timeout: u32,
    ) -> Result<isize, isize> {
        let data_ptr: *mut std::ffi::c_void = data.as_ptr() as *mut std::ffi::c_void;

        let res = unsafe {
            bladerf_sync_tx(
                self.device,
                data_ptr,
                data.len() as u32,
                meta as *mut bladerf_metadata,
                stream_timeout,
            )
        };

        handle_res!(res);
    }

    pub fn sync_tx(&self, data: &[Complex<i16>], stream_timeout: u32) -> Result<isize, isize> {
        let data_ptr: *mut std::ffi::c_void = data.as_ptr() as *mut std::ffi::c_void;

        let res = unsafe {
            bladerf_sync_tx(
                self.device,
                data_ptr,
                data.len() as u32,
                ptr::null_mut(),
                stream_timeout,
            )
        };

        handle_res!(res)
    }

    pub fn sync_rx_meta(
        &self,
        data: &mut [Complex<i16>],
        meta: &mut bladerf_metadata,
        stream_timeout: u32,
    ) -> Result<isize, isize> {
        let data_ptr: *mut std::ffi::c_void = data.as_ptr() as *mut std::ffi::c_void;

        let res = unsafe {
            bladerf_sync_rx(
                self.device,
                data_ptr,
                data.len() as u32,
                meta as *mut bladerf_metadata,
                stream_timeout,
            )
        };

        handle_res!(res)
    }

    pub fn sync_rx(&self, data: &mut [Complex<i16>], stream_timeout: u32) -> Result<isize, isize> {
        let data_ptr: *mut std::ffi::c_void = data.as_ptr() as *mut std::ffi::c_void;

        let res = unsafe {
            bladerf_sync_rx(
                self.device,
                data_ptr,
                data.len() as u32,
                ptr::null_mut(),
                stream_timeout,
            )
        };

        handle_res!(res)
    }

    // Device loading and programming

    pub fn load_fpga(&self, file: String) -> Result<isize, isize> {
        let c_string = ffi::CString::new(file.into_bytes()).unwrap();

        let res = unsafe { bladerf_load_fpga(self.device, c_string.as_ptr()) };

        handle_res!(res)
    }

    /*
    pub fn bladerf_get_bias_tee(dev: *mut bladerf,
        module: bladerf_module,
        enable: *mut bool) -> ::libc::c_int;
    pub fn bladerf_set_bias_tee(dev: *mut bladerf,
            module: bladerf_module,
            enable: bool) -> ::libc::c_int;
    */

    pub fn get_bias_tee(&self, module: bladerf_module) -> Result<bool, isize> {
        let mut value = false;
        let res = unsafe { bladerf_get_bias_tee(self.device, module, &mut value) };
        handle_res!(res, value)
    }

    pub fn set_bias_tee(&self, module: bladerf_module, enable: bool) -> Result<isize, isize> {
        let res = unsafe { bladerf_set_bias_tee(self.device, module, enable) };
        handle_res!(res)
    }

    // Higher level control
    pub fn configure_module(&self, module: BladeRFChannel, config: BladeRFModuleConfig) {
        BladeRF::set_frequency(self, module, config.frequency).unwrap();
        BladeRF::set_sample_rate(self, module as i32, config.sample_rate).unwrap();
        BladeRF::set_bandwidth(self, module as i32, config.bandwidth).unwrap();
        BladeRF::set_gain(self, module as i32, config.lna_gain).unwrap();

        // unsure whether this is still required / doesn't sem correct
        #[cfg(feature = "unimplemented")]
        match module {
            BladeRFChannel::RX0 => {
                BladeRF::set_rxvga1(self, config.vga1).unwrap();
                BladeRF::set_rxvga2(self, config.vga2).unwrap();
            }
            BladeRFChannel::TX0 => {
                BladeRF::set_txvga1(self, config.vga1).unwrap();
                BladeRF::set_txvga2(self, config.vga2).unwrap();
            }
            BladeRFChannel::RX1 => {
                BladeRF::set_rxvga1(self, config.vga1).unwrap();
                BladeRF::set_rxvga2(self, config.vga2).unwrap();
            }
            BladeRFChannel::Tx1 => {
                BladeRF::set_txvga1(self, config.vga1).unwrap();
                BladeRF::set_txvga2(self, config.vga2).unwrap();
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_devices() -> Result<(), isize> {
        match BladeRF::get_device_list() {
            Ok(devices) => {
                println!("Discovered {:?} devices", devices.len());
                Ok(())
            }
            Err(code) => {
                println!("Error {:?} listing devices", code);
                Err(code)
            }
        }
    }

    #[test]
    fn test_open() {
        let _device = BladeRF::open(None).unwrap();
    }

    #[test]
    fn test_open_devinfo() {
        let devices = BladeRF::get_device_list().unwrap();
        assert!(!devices.is_empty());
        let _device = BladeRF::open_with_devinfo(devices[0]).unwrap();
    }

    #[test]
    fn test_get_fw_version() {
        let device = BladeRF::open(None).unwrap();

        let version = device.fw_version().unwrap();
        println!("FW Version {:?}", version);
    }

    #[test]
    fn test_get_fpga_version() {
        let device = BladeRF::open(None).unwrap();

        let version = device.fpga_version().unwrap();
        println!("FPGA Version {:?}", version);
    }

    #[test]
    fn test_get_serial() {
        let device = BladeRF::open(None).unwrap();

        let serial = device.get_serial().unwrap();
        println!("Serial: {:?}", serial);
        assert!(serial.len() == 33);
    }

    #[test]
    fn test_fpga_loaded() {
        let device = BladeRF::open(None).unwrap();

        let loaded = device.is_fpga_configured().unwrap();
        assert!(loaded);
    }

    #[test]
    fn test_loopback_modes() {
        let device = BladeRF::open(None).unwrap();

        // Check initial is none
        let loopback = device.get_loopback().unwrap();
        assert!(loopback == BladeRFLoopback::None);

        // Set and check loopback modes
        device.set_loopback(BladeRFLoopback::Firmware).unwrap();
        let loopback = device.get_loopback().unwrap();
        assert!(loopback == BladeRFLoopback::Firmware);

        // Reset
        device.set_loopback(BladeRFLoopback::None).unwrap();

        let loopback = device.get_loopback().unwrap();
        assert!(loopback == BladeRFLoopback::None);
    }

    #[test]
    fn test_set_freq() {
        let device = BladeRF::open(None).unwrap();

        let freq: u64 = 915000000;

        // Set and check frequency
        device.set_frequency(BladeRFChannel::Rx1, freq).unwrap();
        let actual_freq = device.get_frequency(BladeRFChannel::Rx1).unwrap();
        let diff = freq as i64 - actual_freq as i64;
        assert!(i64::abs(diff) < 10);
    }

    #[test]
    fn test_set_sampling() {
        let device = BladeRF::open(None).unwrap();

        let sampling: bladerf_sampling = bladerf_sampling_BLADERF_SAMPLING_INTERNAL;

        // Set and check frequency
        match device.set_sampling(sampling) {
            Ok(_) => (),
            Err(err) => {
                if err != -8 {
                    panic!(
                        "unexpected error of value when calling set_sampling {:?}",
                        err
                    );
                } else {
                    return;
                }
            }
        };

        let actual_sampling = device.get_sampling().unwrap();

        assert!(actual_sampling == sampling);
    }
}
