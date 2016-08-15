/* automatically generated by rust-bindgen */

extern crate libc;

use libc::*;
use std::*;


pub enum Struct_bladerf { }

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum bladerf_backend {
    BLADERF_BACKEND_ANY = 0,    
    BLADERF_BACKEND_LINUX = 1,  
    BLADERF_BACKEND_LIBUSB = 2, 
    BLADERF_BACKEND_CYPRESS = 3, 
    BLADERF_BACKEND_DUMMY = 100,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum bladerf_dev_speed {
    BLADERF_DEVICE_SPEED_UNKNOWN = 0,
    BLADERF_DEVICE_SPEED_HIGH = 1,
    BLADERF_DEVICE_SPEED_SUPER = 2,
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_bladerf_devinfo {
    pub backend: bladerf_backend,
    pub serial: [::libc::c_char; 33usize],
    pub usb_bus: uint8_t,
    pub usb_addr: uint8_t,
    pub instance: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_bladerf_devinfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_bladerf_devinfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

impl Struct_bladerf_devinfo {
  pub fn serial(&self) -> String {
      let serial_u8: Vec<u8>= self.serial.iter().map(|&x| x as u8).collect();

      // Build String
      let serial_cstr = unsafe { ffi::CString::from_vec_unchecked(serial_u8) };
      let serial_str = serial_cstr.into_string().unwrap();

      return serial_str;
  }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum bladerf_tuning_mode {
    BLADERF_TUNING_MODE_INVALID = -1,
    BLADERF_TUNING_MODE_HOST = 0,
    BLADERF_TUNING_MODE_FPGA = 1,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum bladerf_loopback {
    BLADERF_LB_FIRMWARE = 1,
    BLADERF_LB_BB_TXLPF_RXVGA2 = 2,
    BLADERF_LB_BB_TXVGA1_RXVGA2 = 3,
    BLADERF_LB_BB_TXLPF_RXLPF = 4,
    BLADERF_LB_BB_TXVGA1_RXLPF = 5,
    BLADERF_LB_RF_LNA1 = 6,
    BLADERF_LB_RF_LNA2 = 7,
    BLADERF_LB_RF_LNA3 = 8,
    BLADERF_LB_NONE = 9,
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_bladerf_rational_rate {
    pub integer: uint64_t,
    pub num: uint64_t,
    pub den: uint64_t,
}
impl ::std::clone::Clone for Struct_bladerf_rational_rate {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_bladerf_rational_rate {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, PartialEq)]
pub enum bladerf_sampling {
    BLADERF_SAMPLING_UNKNOWN = 0,
    BLADERF_SAMPLING_INTERNAL = 1,
    BLADERF_SAMPLING_EXTERNAL = 2,
}
impl ::std::clone::Clone for bladerf_sampling {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for bladerf_sampling {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
pub enum bladerf_lna_gain {
    BLADERF_LNA_GAIN_UNKNOWN = 0,
    BLADERF_LNA_GAIN_BYPASS = 1,
    BLADERF_LNA_GAIN_MID = 2,
    BLADERF_LNA_GAIN_MAX = 3,
}

#[repr(C)]
#[derive(Copy)]
pub enum bladerf_lpf_mode {
    BLADERF_LPF_NORMAL = 0,
    BLADERF_LPF_BYPASSED = 1,
    BLADERF_LPF_DISABLED = 2,
}
impl ::std::clone::Clone for bladerf_lpf_mode {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Copy)]
pub enum bladerf_module {
    BLADERF_MODULE_RX = 0,
    BLADERF_MODULE_TX = 1,
}
impl ::std::clone::Clone for bladerf_module {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
pub enum bladerf_xb {
    BLADERF_XB_NONE = 0,
    BLADERF_XB_100 = 1,
    BLADERF_XB_200 = 2,
}

#[repr(C)]
pub enum bladerf_xb200_filter {
    BLADERF_XB200_50M = 0,
    BLADERF_XB200_144M = 1,
    BLADERF_XB200_222M = 2,
    BLADERF_XB200_CUSTOM = 3,
    BLADERF_XB200_AUTO_1DB = 4,
    BLADERF_XB200_AUTO_3DB = 5,
}

#[repr(C)]
pub enum bladerf_xb200_path {
    BLADERF_XB200_BYPASS = 0,
    BLADERF_XB200_MIX = 1,
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_bladerf_quick_tune {
    pub freqsel: uint8_t,
    pub vcocap: uint8_t,
    pub nint: uint16_t,
    pub nfrac: uint32_t,
    pub flags: uint8_t,
}
impl ::std::clone::Clone for Struct_bladerf_quick_tune {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_bladerf_quick_tune {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
pub enum bladerf_cal_module {
    BLADERF_DC_CAL_LPF_TUNING = 0,
    BLADERF_DC_CAL_TX_LPF = 1,
    BLADERF_DC_CAL_RX_LPF = 2,
    BLADERF_DC_CAL_RXVGA2 = 3,
}

#[repr(C)]
pub enum bladerf_correction {
    BLADERF_CORR_LMS_DCOFF_I = 0,
    BLADERF_CORR_LMS_DCOFF_Q = 1,
    BLADERF_CORR_FPGA_PHASE = 2,
    BLADERF_CORR_FPGA_GAIN = 3,
}

#[repr(C)]
pub enum bladerf_format {
    BLADERF_FORMAT_SC16_Q11 = 0,
    BLADERF_FORMAT_SC16_Q11_META = 1,
}

pub enum bladerf_error {
    BLADERF_ERR_UNEXPECTED  = -1,
    BLADERF_ERR_RANGE       = -2,
    BLADERF_ERR_INVAL       = -3,
    BLADERF_ERR_MEM         = -4,
    BLADERF_ERR_IO          = -5,
    BLADERF_ERR_TIMEOUT     = -6,
    BLADERF_ERR_NODEV       = -7,
    BLADERF_ERR_UNSUPPORTED = -8,
    BLADERF_ERR_MISALIGNED  = -9,
    BLADERF_ERR_CHECKSUM    = -10,
    BLADERF_ERR_NO_FILE     = -11,
    BLADERF_ERR_UPDATE_FPGA = -12,
    BLADERF_ERR_UPDATE_FW   = -13,
    BLADERF_ERR_TIME_PAST   = -14,
    BLADERF_ERR_QUEUE_FULL  = -15,
    BLADERF_ERR_FPGA_OP     = -16,
    BLADERF_ERR_PERMISSION  = -17,
    BLADERF_ERR_WOULD_BLOCK = -18,
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_bladerf_metadata {
    pub timestamp: uint64_t,
    pub flags: uint32_t,
    pub status: uint32_t,
    pub actual_count: ::libc::c_uint,
    pub reserved: [uint8_t; 32usize],
}
impl ::std::clone::Clone for Struct_bladerf_metadata {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_bladerf_metadata {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub enum Struct_bladerf_stream { }
pub type bladerf_stream_cb =
    ::std::option::Option<extern "C" fn(dev: *mut Struct_bladerf,
                                        stream: *mut Struct_bladerf_stream,
                                        meta: *mut Struct_bladerf_metadata,
                                        samples: *mut ::libc::c_void,
                                        num_samples: size_t,
                                        user_data: *mut ::libc::c_void)
                              -> *mut ::libc::c_void>;

#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_bladerf_version {
    pub major: uint16_t,
    pub minor: uint16_t,
    pub patch: uint16_t,
    pub describe: *const ::libc::c_char,
}
impl ::std::clone::Clone for Struct_bladerf_version {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_bladerf_version {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
pub enum bladerf_fpga_size {
    BLADERF_FPGA_UNKNOWN = 0,
    BLADERF_FPGA_40KLE = 40,
    BLADERF_FPGA_115KLE = 115,
}

#[repr(C)]
pub enum bladerf_log_level {
    BLADERF_LOG_LEVEL_VERBOSE = 0,
    BLADERF_LOG_LEVEL_DEBUG = 1,
    BLADERF_LOG_LEVEL_INFO = 2,
    BLADERF_LOG_LEVEL_WARNING = 3,
    BLADERF_LOG_LEVEL_ERROR = 4,
    BLADERF_LOG_LEVEL_CRITICAL = 5,
    BLADERF_LOG_LEVEL_SILENT = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bladerf_image_type {
    BLADERF_IMAGE_TYPE_INVALID= -1,
    BLADERF_IMAGE_TYPE_RAW = 0,
    BLADERF_IMAGE_TYPE_FIRMWARE = 1,
    BLADERF_IMAGE_TYPE_FPGA_40KLE = 2,
    BLADERF_IMAGE_TYPE_FPGA_115KLE = 3,
    BLADERF_IMAGE_TYPE_CALIBRATION = 4,
    BLADERF_IMAGE_TYPE_RX_DC_CAL = 5,
    BLADERF_IMAGE_TYPE_TX_DC_CAL = 6,
    BLADERF_IMAGE_TYPE_RX_IQ_CAL = 7,
    BLADERF_IMAGE_TYPE_TX_IQ_CAL = 8,
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_bladerf_image {
    pub magic: [::libc::c_char; 8usize],
    pub checksum: [uint8_t; 32usize],
    pub version: Struct_bladerf_version,
    pub timestamp: uint64_t,
    pub serial: [::libc::c_char; 34usize],
    pub reserved: [::libc::c_char; 128usize],
    pub _type: bladerf_image_type,
    pub address: uint32_t,
    pub length: uint32_t,
    pub data: *mut uint8_t,
}
impl ::std::clone::Clone for Struct_bladerf_image {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_bladerf_image {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_bladerf_lms_dc_cals {
    pub lpf_tuning: int16_t,
    pub tx_lpf_i: int16_t,
    pub tx_lpf_q: int16_t,
    pub rx_lpf_i: int16_t,
    pub rx_lpf_q: int16_t,
    pub dc_ref: int16_t,
    pub rxvga2a_i: int16_t,
    pub rxvga2a_q: int16_t,
    pub rxvga2b_i: int16_t,
    pub rxvga2b_q: int16_t,
}
impl ::std::clone::Clone for Struct_bladerf_lms_dc_cals {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_bladerf_lms_dc_cals {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[link(name = "bladeRF")]
extern "C" {
    pub fn bladerf_get_device_list(devices: &*mut Struct_bladerf_devinfo) 
    -> libc::c_int;
    pub fn bladerf_free_device_list(devices: *mut Struct_bladerf_devinfo)
    -> ();
    pub fn bladerf_open_with_devinfo(device: &*mut Struct_bladerf,
                                     devinfo: *const Struct_bladerf_devinfo)
     -> ::libc::c_int;
    pub fn bladerf_open(device:  &*mut Struct_bladerf,
                        device_identifier: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn bladerf_close(device: *mut Struct_bladerf) -> ();
    pub fn bladerf_set_usb_reset_on_open(enabled: u8) -> ();
    pub fn bladerf_init_devinfo(info: *mut Struct_bladerf_devinfo) -> ();
    pub fn bladerf_get_devinfo(dev: *mut Struct_bladerf,
                               info: *mut Struct_bladerf_devinfo)
     -> ::libc::c_int;
    pub fn bladerf_get_devinfo_from_str(devstr: *const ::libc::c_char,
                                        info: *mut Struct_bladerf_devinfo)
     -> ::libc::c_int;
    pub fn bladerf_devinfo_matches(a: *const Struct_bladerf_devinfo,
                                   b: *const Struct_bladerf_devinfo) -> u8;
    pub fn bladerf_devstr_matches(dev_str: *const ::libc::c_char,
                                  info: *mut Struct_bladerf_devinfo) -> u8;
    pub fn bladerf_backend_str(backend: bladerf_backend)
     -> *const ::libc::c_char;
    pub fn bladerf_enable_module(dev: *mut Struct_bladerf, m: bladerf_module,
                                 enable: u8) -> ::libc::c_int;
    pub fn bladerf_set_loopback(dev: *mut Struct_bladerf, l: bladerf_loopback)
     -> ::libc::c_int;
    pub fn bladerf_get_loopback(dev: *mut Struct_bladerf,
                                l: *mut bladerf_loopback) -> ::libc::c_int;
    pub fn bladerf_set_sample_rate(dev: *mut Struct_bladerf,
                                   module: bladerf_module,
                                   rate: ::libc::c_uint,
                                   actual: *mut ::libc::c_uint)
     -> ::libc::c_int;
    pub fn bladerf_set_rational_sample_rate(dev: *mut Struct_bladerf,
                                            module: bladerf_module,
                                            rate:
                                                *mut Struct_bladerf_rational_rate,
                                            actual:
                                                *mut Struct_bladerf_rational_rate)
     -> ::libc::c_int;
    pub fn bladerf_set_sampling(dev: *mut Struct_bladerf,
                                sampling: bladerf_sampling) -> ::libc::c_int;
    pub fn bladerf_get_sampling(dev: *mut Struct_bladerf,
                                sampling: *mut bladerf_sampling)
     -> ::libc::c_int;
    pub fn bladerf_get_sample_rate(dev: *mut Struct_bladerf,
                                   module: bladerf_module,
                                   rate: *mut ::libc::c_uint)
     -> ::libc::c_int;
    pub fn bladerf_get_rational_sample_rate(dev: *mut Struct_bladerf,
                                            module: bladerf_module,
                                            rate:
                                                *mut Struct_bladerf_rational_rate)
     -> ::libc::c_int;
    pub fn bladerf_set_correction(dev: *mut Struct_bladerf,
                                  module: bladerf_module,
                                  corr: bladerf_correction, value: int16_t)
     -> ::libc::c_int;
    pub fn bladerf_get_correction(dev: *mut Struct_bladerf,
                                  module: bladerf_module,
                                  corr: bladerf_correction,
                                  value: *mut int16_t) -> ::libc::c_int;
    pub fn bladerf_set_txvga2(dev: *mut Struct_bladerf, gain: ::libc::c_int)
     -> ::libc::c_int;
    pub fn bladerf_get_txvga2(dev: *mut Struct_bladerf,
                              gain: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn bladerf_set_txvga1(dev: *mut Struct_bladerf, gain: ::libc::c_int)
     -> ::libc::c_int;
    pub fn bladerf_get_txvga1(dev: *mut Struct_bladerf,
                              gain: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn bladerf_set_lna_gain(dev: *mut Struct_bladerf,
                                gain: bladerf_lna_gain) -> ::libc::c_int;
    pub fn bladerf_get_lna_gain(dev: *mut Struct_bladerf,
                                gain: *mut bladerf_lna_gain) -> ::libc::c_int;
    pub fn bladerf_set_rxvga1(dev: *mut Struct_bladerf, gain: ::libc::c_int)
     -> ::libc::c_int;
    pub fn bladerf_get_rxvga1(dev: *mut Struct_bladerf,
                              gain: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn bladerf_set_rxvga2(dev: *mut Struct_bladerf, gain: ::libc::c_int)
     -> ::libc::c_int;
    pub fn bladerf_get_rxvga2(dev: *mut Struct_bladerf,
                              gain: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn bladerf_set_gain(dev: *mut Struct_bladerf, _mod: bladerf_module,
                            gain: ::libc::c_int) -> ::libc::c_int;
    pub fn bladerf_set_bandwidth(dev: *mut Struct_bladerf,
                                 module: bladerf_module,
                                 bandwidth: ::libc::c_uint,
                                 actual: *mut ::libc::c_uint)
     -> ::libc::c_int;
    pub fn bladerf_get_bandwidth(dev: *mut Struct_bladerf,
                                 module: bladerf_module,
                                 bandwidth: *mut ::libc::c_uint)
     -> ::libc::c_int;
    pub fn bladerf_set_lpf_mode(dev: *mut Struct_bladerf,
                                module: bladerf_module,
                                mode: bladerf_lpf_mode) -> ::libc::c_int;
    pub fn bladerf_get_lpf_mode(dev: *mut Struct_bladerf,
                                module: bladerf_module,
                                mode: *mut bladerf_lpf_mode) -> ::libc::c_int;
    pub fn bladerf_select_band(dev: *mut Struct_bladerf,
                               module: bladerf_module,
                               frequency: ::libc::c_uint) -> ::libc::c_int;
    pub fn bladerf_set_frequency(dev: *mut Struct_bladerf,
                                 module: bladerf_module,
                                 frequency: ::libc::c_uint) -> ::libc::c_int;
    pub fn bladerf_schedule_retune(dev: *mut Struct_bladerf,
                                   module: bladerf_module,
                                   timestamp: uint64_t,
                                   frequency: ::libc::c_uint,
                                   quick_tune: *mut Struct_bladerf_quick_tune)
     -> ::libc::c_int;
    pub fn bladerf_cancel_scheduled_retunes(dev: *mut Struct_bladerf,
                                            module: bladerf_module)
     -> ::libc::c_int;
    pub fn bladerf_get_frequency(dev: *mut Struct_bladerf,
                                 module: bladerf_module,
                                 frequency: *mut ::libc::c_uint)
     -> ::libc::c_int;
    pub fn bladerf_get_quick_tune(dev: *mut Struct_bladerf,
                                  module: bladerf_module,
                                  quick_tune: *mut Struct_bladerf_quick_tune)
     -> ::libc::c_int;
    pub fn bladerf_set_tuning_mode(dev: *mut Struct_bladerf,
                                   mode: bladerf_tuning_mode)
     -> ::libc::c_int;
    pub fn bladerf_expansion_attach(dev: *mut Struct_bladerf, xb: bladerf_xb)
     -> ::libc::c_int;
    pub fn bladerf_expansion_get_attached(dev: *mut Struct_bladerf,
                                          xb: *mut bladerf_xb)
     -> ::libc::c_int;
    pub fn bladerf_xb200_set_filterbank(dev: *mut Struct_bladerf,
                                        _mod: bladerf_module,
                                        filter: bladerf_xb200_filter)
     -> ::libc::c_int;
    pub fn bladerf_xb200_get_filterbank(dev: *mut Struct_bladerf,
                                        module: bladerf_module,
                                        filter: *mut bladerf_xb200_filter)
     -> ::libc::c_int;
    pub fn bladerf_xb200_set_path(dev: *mut Struct_bladerf,
                                  module: bladerf_module,
                                  path: bladerf_xb200_path) -> ::libc::c_int;
    pub fn bladerf_xb200_get_path(dev: *mut Struct_bladerf,
                                  module: bladerf_module,
                                  path: *mut bladerf_xb200_path)
     -> ::libc::c_int;
    pub fn bladerf_init_stream(stream: *mut *mut Struct_bladerf_stream,
                               dev: *mut Struct_bladerf,
                               callback: bladerf_stream_cb,
                               buffers: *mut *mut *mut ::libc::c_void,
                               num_buffers: size_t, format: bladerf_format,
                               samples_per_buffer: size_t,
                               num_transfers: size_t,
                               user_data: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn bladerf_stream(stream: *mut Struct_bladerf_stream,
                          module: bladerf_module) -> ::libc::c_int;
    pub fn bladerf_submit_stream_buffer(stream: *mut Struct_bladerf_stream,
                                        buffer: *mut ::libc::c_void,
                                        timeout_ms: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn bladerf_deinit_stream(stream: *mut Struct_bladerf_stream) -> ();
    pub fn bladerf_set_stream_timeout(dev: *mut Struct_bladerf,
                                      module: bladerf_module,
                                      timeout: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn bladerf_get_stream_timeout(dev: *mut Struct_bladerf,
                                      module: bladerf_module,
                                      timeout: *mut ::libc::c_uint)
     -> ::libc::c_int;
    pub fn bladerf_sync_config(dev: *mut Struct_bladerf,
                               module: bladerf_module, format: bladerf_format,
                               num_buffers: ::libc::c_uint,
                               buffer_size: ::libc::c_uint,
                               num_transfers: ::libc::c_uint,
                               stream_timeout: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn bladerf_sync_tx(dev: *mut Struct_bladerf,
                           samples: *mut ::libc::c_void,
                           num_samples: ::libc::c_uint,
                           metadata: *mut Struct_bladerf_metadata,
                           timeout_ms: ::libc::c_uint) -> ::libc::c_int;
    pub fn bladerf_sync_rx(dev: *mut Struct_bladerf,
                           samples: *mut ::libc::c_void,
                           num_samples: ::libc::c_uint,
                           metadata: *mut Struct_bladerf_metadata,
                           timeout_ms: ::libc::c_uint) -> ::libc::c_int;
    pub fn bladerf_get_serial(dev: *mut Struct_bladerf,
                              serial: *const ::libc::c_char) -> ::libc::c_int;
    pub fn bladerf_get_vctcxo_trim(dev: *mut Struct_bladerf,
                                   trim: *mut uint16_t) -> ::libc::c_int;
    pub fn bladerf_get_fpga_size(dev: *mut Struct_bladerf,
                                 size: *mut bladerf_fpga_size)
     -> ::libc::c_int;
    pub fn bladerf_fw_version(dev: *mut Struct_bladerf,
                              version: *mut Struct_bladerf_version)
     -> ::libc::c_int;
    pub fn bladerf_is_fpga_configured(dev: *mut Struct_bladerf)
     -> ::libc::c_int;
    pub fn bladerf_fpga_version(dev: *mut Struct_bladerf,
                                version: *mut Struct_bladerf_version)
     -> ::libc::c_int;
    pub fn bladerf_device_speed(dev: *mut Struct_bladerf)
     -> bladerf_dev_speed;
    pub fn bladerf_flash_firmware(dev: *mut Struct_bladerf,
                                  firmware: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn bladerf_load_fpga(dev: *mut Struct_bladerf,
                             fpga: *const ::libc::c_char) -> ::libc::c_int;
    pub fn bladerf_flash_fpga(dev: *mut Struct_bladerf,
                              fpga_image: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn bladerf_erase_stored_fpga(dev: *mut Struct_bladerf)
     -> ::libc::c_int;
    pub fn bladerf_device_reset(dev: *mut Struct_bladerf) -> ::libc::c_int;
    pub fn bladerf_jump_to_bootloader(dev: *mut Struct_bladerf)
     -> ::libc::c_int;
    pub fn bladerf_strerror(error: ::libc::c_int) -> *const ::libc::c_char;
    pub fn bladerf_version(version: *mut Struct_bladerf_version) -> ();
    pub fn bladerf_log_set_verbosity(level: bladerf_log_level) -> ();
    pub fn bladerf_alloc_image(_type: bladerf_image_type, address: uint32_t,
                               length: uint32_t) -> *mut Struct_bladerf_image;
    pub fn bladerf_alloc_cal_image(fpga_size: bladerf_fpga_size,
                                   vctcxo_trim: uint16_t)
     -> *mut Struct_bladerf_image;
    pub fn bladerf_free_image(image: *mut Struct_bladerf_image) -> ();
    pub fn bladerf_image_write(image: *mut Struct_bladerf_image,
                               file: *const ::libc::c_char) -> ::libc::c_int;
    pub fn bladerf_image_read(image: *mut Struct_bladerf_image,
                              file: *const ::libc::c_char) -> ::libc::c_int;
    pub fn bladerf_si5338_read(dev: *mut Struct_bladerf, address: uint8_t,
                               val: *mut uint8_t) -> ::libc::c_int;
    pub fn bladerf_si5338_write(dev: *mut Struct_bladerf, address: uint8_t,
                                val: uint8_t) -> ::libc::c_int;
    pub fn bladerf_si5338_set_tx_freq(dev: *mut Struct_bladerf,
                                      freq: ::libc::c_uint) -> ::libc::c_int;
    pub fn bladerf_si5338_set_rx_freq(dev: *mut Struct_bladerf,
                                      freq: ::libc::c_uint) -> ::libc::c_int;
    pub fn bladerf_lms_read(dev: *mut Struct_bladerf, address: uint8_t,
                            val: *mut uint8_t) -> ::libc::c_int;
    pub fn bladerf_lms_write(dev: *mut Struct_bladerf, address: uint8_t,
                             val: uint8_t) -> ::libc::c_int;
    pub fn bladerf_lms_set_dc_cals(dev: *mut Struct_bladerf,
                                   dc_cals: *const Struct_bladerf_lms_dc_cals)
     -> ::libc::c_int;
    pub fn bladerf_lms_get_dc_cals(dev: *mut Struct_bladerf,
                                   dc_cals: *mut Struct_bladerf_lms_dc_cals)
     -> ::libc::c_int;
    pub fn bladerf_config_gpio_read(dev: *mut Struct_bladerf,
                                    val: *mut uint32_t) -> ::libc::c_int;
    pub fn bladerf_config_gpio_write(dev: *mut Struct_bladerf, val: uint32_t)
     -> ::libc::c_int;
    pub fn bladerf_expansion_gpio_read(dev: *mut Struct_bladerf,
                                       val: *mut uint32_t) -> ::libc::c_int;
    pub fn bladerf_expansion_gpio_write(dev: *mut Struct_bladerf,
                                        val: uint32_t) -> ::libc::c_int;
    pub fn bladerf_expansion_gpio_dir_read(dev: *mut Struct_bladerf,
                                           val: *mut uint32_t)
     -> ::libc::c_int;
    pub fn bladerf_expansion_gpio_dir_write(dev: *mut Struct_bladerf,
                                            val: uint32_t) -> ::libc::c_int;
    pub fn bladerf_get_timestamp(dev: *mut Struct_bladerf,
                                 _mod: bladerf_module, value: *mut uint64_t)
     -> ::libc::c_int;
    pub fn bladerf_dac_write(dev: *mut Struct_bladerf, val: uint16_t)
     -> ::libc::c_int;
    pub fn bladerf_dac_read(dev: *mut Struct_bladerf, val: *mut uint16_t)
     -> ::libc::c_int;
    pub fn bladerf_xb_spi_write(dev: *mut Struct_bladerf, val: uint32_t)
     -> ::libc::c_int;
    pub fn bladerf_calibrate_dc(dev: *mut Struct_bladerf,
                                module: bladerf_cal_module) -> ::libc::c_int;
    pub fn bladerf_erase_flash(dev: *mut Struct_bladerf,
                               erase_block: uint32_t, count: uint32_t)
     -> ::libc::c_int;
    pub fn bladerf_read_flash(dev: *mut Struct_bladerf, buf: *mut uint8_t,
                              page: uint32_t, count: uint32_t)
     -> ::libc::c_int;
    pub fn bladerf_write_flash(dev: *mut Struct_bladerf, buf: *const uint8_t,
                               page: uint32_t, count: uint32_t)
     -> ::libc::c_int;
    pub fn bladerf_get_bootloader_list(list: *mut *mut Struct_bladerf_devinfo)
     -> ::libc::c_int;
    pub fn bladerf_load_fw_from_bootloader(device_identifier:
                                               *const ::libc::c_char,
                                           backend: bladerf_backend,
                                           bus: uint8_t, addr: uint8_t,
                                           file: *const ::libc::c_char)
     -> ::libc::c_int;
}
