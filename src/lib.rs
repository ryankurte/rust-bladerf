extern crate libc;

use std::*;
use std::mem::*;

#[allow(dead_code, non_camel_case_types)]
mod bladerf;
use bladerf::*;

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
pub struct BladeRFModuleConfig {
	pub frequency: u64,
	pub sample_rate: u32,
	pub bandwidth: u32,
	pub lna_gain: bladerf_lna_gain,
	pub vga1: i32,
	pub vga2: i32
}

// BladeRF overall config object
pub struct BladeRFConfig {
	pub tx: BladeRFModuleConfig,
	pub rx: BladeRFModuleConfig
}

#[repr(C)]
#[repr(packed)]
pub struct iq {
	pub i: i16,
	pub q: i16
}

// BladeRF device object
pub struct BladeRFDevice {
	device: MaybeUninit<*mut Struct_bladerf>,
}

impl Drop for BladeRFDevice {
	fn drop(&mut self) {
		unsafe {
			bladerf_close(self.device.assume_init())
		}
	}
}


/***		Static Functions			***/

pub fn get_device_list() -> Result<Vec<Struct_bladerf_devinfo>, isize> {

	unsafe{ 
		let mut devices = MaybeUninit::<*mut Struct_bladerf_devinfo>::uninit();

		let n = bladerf_get_device_list(devices.as_mut_ptr()) as isize;

		// Catch bladerf function errors
		if n > 0 {

			// Cast array to slice and create a safe array to return
			let device_slice = std::slice::from_raw_parts(*devices.as_ptr(), n as usize);
			let mut safe_device_list: Vec<Struct_bladerf_devinfo> = Vec::new();

			for i in 0..n {
				let local_device = device_slice[i as usize];
				//Safe if this is a copy, unsafe if it is not?
				safe_device_list.push(local_device);
			}
			bladerf_free_device_list(*devices.as_ptr());
			
			// Return rust save device info array
			Ok(safe_device_list)

		} else {
			// Return error code
			Err(n)
		}
	}
}

pub fn set_usb_reset_on_open(enabled: bool) {
    unsafe{ 
    	bladerf_set_usb_reset_on_open(enabled as libc::uint8_t); 
    } 
}

pub fn open(identifier: Option<String>) -> Result<BladeRFDevice, isize> {
	unsafe {
		let id_ptr = match identifier {
			Some(id) => {
				let c_string = ffi::CString::new(id.into_bytes()).unwrap();
				c_string.as_ptr()
			}, None => {
				ptr::null()
			}
		};

		let mut bladerf_device = BladeRFDevice { device: MaybeUninit::uninit() };

		let res = bladerf_open(bladerf_device.device.as_mut_ptr(), id_ptr);

		handle_res!(res, bladerf_device);
	}
}

pub fn open_with_devinfo(devinfo: &Struct_bladerf_devinfo) -> Result<BladeRFDevice, isize> {

	let devinfo_ptr: *const Struct_bladerf_devinfo = devinfo as *const Struct_bladerf_devinfo;

	unsafe {
		let mut bladerf_device = BladeRFDevice { device: MaybeUninit::uninit() };

		let res = bladerf_open_with_devinfo(bladerf_device.device.as_mut_ptr(), devinfo_ptr);

		handle_res!(res, bladerf_device);
	}
}


/***		BladeRFDevice Methods			***/

impl BladeRFDevice {

	// Device Properties and Information
	// http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___i_n_f_o.html

	pub fn get_serial(&self) -> Result<String, isize> {
		unsafe {
			// Create raw data array for serial return
			let serial_data : Vec<::libc::c_char> = vec![0; 33];

			// Call underlying c method
			let res = bladerf_get_serial(self.device.assume_init(), serial_data.as_ptr());

			if res >= 0 {
				// Map ::libc::c_char back to u8 as required for string manipulation
				let serial_u8: Vec<u8>= serial_data.iter().map(|&x| x as u8).collect();

				// Build String
				let serial_cstr = ffi::CString::from_vec_unchecked(serial_u8);
				let serial_str = serial_cstr.into_string().unwrap();

				Ok(serial_str)
			} else {
				Err(res as isize)
			}
		}
	}

	pub fn get_fpga_size(&self) -> Result<bladerf_fpga_size, isize> {
		let mut fpga_size: bladerf_fpga_size = bladerf_fpga_size::BLADERF_FPGA_UNKNOWN;

		unsafe {
			let res = bladerf_get_fpga_size(self.device.assume_init(), &mut fpga_size);

			handle_res!(res, fpga_size);
		}
	}

	pub fn fw_version(&self) -> Result<Struct_bladerf_version, isize> {
		unsafe {
			let mut version = Struct_bladerf_version {
				major: 0,
				minor: 0,
				patch: 0,
				describe: 0 as *const i8,
			};

			let res = bladerf_fw_version(
				self.device.assume_init(),
				&mut version
			);

			handle_res!(res, version);
		}
	}

	pub fn is_fpga_configured(&self) -> Result<bool, isize> {
		unsafe {
			let res = bladerf_is_fpga_configured(self.device.assume_init());

			if res > 0 {
				Ok(true)
			} else if res == 0 {
				Ok(false)
			} else {
				Err(res as isize)
			}
		}
	}

	pub fn fpga_version(&self) -> Result<Struct_bladerf_version, isize> {
		unsafe {
			let mut version = Struct_bladerf_version {
				major: 0,
				minor: 0,
				patch: 0,
				describe: 0 as *const i8,
			};			

			let res = bladerf_fpga_version(self.device.assume_init(), &mut version);

			handle_res!(res, version);
		}
	}

	// RX & TX Module Control
	// http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___m_o_d_u_l_e.html

	pub fn enable_module(&self, module: bladerf_module, enable: bool) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_enable_module(self.device.assume_init(), module, enable as u8);

			handle_res!(res);
		}
	}


	// Gain Control
	// http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___g_a_i_n.html

	pub fn set_lna_gain(&self, gain: bladerf_lna_gain) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_lna_gain(self.device.assume_init(), gain);

			handle_res!(res);
		}
	}

	pub fn get_lna_gain(&self) -> Result<bladerf_lna_gain, isize> {
		unsafe {
			let mut gain: bladerf_lna_gain = bladerf_lna_gain::BLADERF_LNA_GAIN_UNKNOWN;

			let res = bladerf_get_lna_gain(self.device.assume_init(), &mut gain); 

			handle_res!(res, gain);
		}
	}

	pub fn set_rxvga1(&self, gain: i32) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_rxvga1(self.device.assume_init(), gain);

			handle_res!(res);
		}
	}

	pub fn get_rxvga1(&self) -> Result<i32, isize> {
		unsafe {
			let mut gain: i32 = 0;

			let res = bladerf_get_rxvga1(self.device.assume_init(), &mut gain); 

			handle_res!(res, gain);
		}
	}

	pub fn set_rxvga2(&self, gain: i32) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_rxvga2(self.device.assume_init(), gain);

			handle_res!(res);
		}
	}

	pub fn get_rxvga2(&self) -> Result<i32, isize> {
		unsafe {
			let mut gain: i32 = 0;

			let res = bladerf_get_rxvga2(self.device.assume_init(), &mut gain); 

			handle_res!(res, gain);
		}
	}

	pub fn set_txvga1(&self, gain: i32) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_txvga1(self.device.assume_init(), gain);

			handle_res!(res);
		}
	}

	pub fn get_txvga1(&self) -> Result<i32, isize> {
		unsafe {
			let mut gain: i32 = 0;

			let res = bladerf_get_txvga1(self.device.assume_init(), &mut gain); 

			handle_res!(res, gain);
		}
	}

	pub fn set_txvga2(&self, gain: i32) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_txvga2(self.device.assume_init(), gain);

			handle_res!(res);
		}
	}

	pub fn get_txvga2(&self) -> Result<i32, isize> {
		unsafe {
			let mut gain: i32 = 0;

			let res = bladerf_get_txvga2(self.device.assume_init(), &mut gain); 

			handle_res!(res, gain);
		}
	}

	pub fn set_gain(&self, module: bladerf_module, gain: i32) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_gain(self.device.assume_init(), module, gain);

			handle_res!(res);
		}
	}

	// Sampling Control

	pub fn set_sample_rate(&self, module: bladerf_module, rate: u32) -> Result<u32, isize> {
		let mut actual: u32 = 0;

		unsafe {
			let res = bladerf_set_sample_rate(self.device.assume_init(), module, rate, &mut actual);

			handle_res!(res, actual);
		}
	}

	pub fn set_rational_sample_rate(&self, module: bladerf_module, rate: Struct_bladerf_rational_rate) -> Result<Struct_bladerf_rational_rate, isize> {
		let mut rate = rate;

		unsafe {
			let mut actual = Struct_bladerf_rational_rate {
				integer: 0,
				num: 0,
				den: 0,
			};

			let res = bladerf_set_rational_sample_rate(self.device.assume_init(),
														module,
														&mut rate,
														&mut actual);
			handle_res!(res, actual);
		}
	}

	pub fn get_sample_rate(&self, module: bladerf_module) -> Result<u32, isize> {
		let mut rate: u32 = 0;

		unsafe {
			let res = bladerf_get_sample_rate(self.device.assume_init(), module, &mut rate);

			handle_res!(res, rate);
		}
	}

	pub fn get_rational_sample_rate(&self, module: bladerf_module) -> Result<Struct_bladerf_rational_rate, isize> {
		unsafe {
			let mut rate = Struct_bladerf_rational_rate {
				integer: 0,
				num: 0,
				den: 0,
			};

			let res = bladerf_get_rational_sample_rate(self.device.assume_init(), module, &mut rate);
			
			handle_res!(res, rate);
		}
	}

	pub fn set_sampling(&self, sampling: bladerf_sampling) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_sampling(self.device.assume_init(), sampling);

			handle_res!(res);
		}
	}
/*
	Generated bladerf.rs needs update

	pub fn set_rx_mux(&self, mux: bladerf_rx_mux) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_rx_mux(self.device, sampling);

			handle_res!(res);
		}
	}

	pub fn get_rx_mux(&self, ) -> Result<bladerf_rx_mux, isize> {
		let mut mux: bladerf_rx_mux = 0;

		unsafe {
			let res = bladerf_get_rx_mux(self.device, &mut mux as *mut bladerf_rx_mux);

			handle_res!(res, mux);
		}
	}
*/

	pub fn get_sampling(&self) -> Result<bladerf_sampling, isize> {
		unsafe {
			let mut sampling = bladerf_sampling::BLADERF_SAMPLING_UNKNOWN;

			let res = bladerf_get_sampling(self.device.assume_init(), &mut sampling);

			handle_res!(res, sampling);
		}
	}

	// Bandwidth Configuration
	// http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___b_a_n_d_w_i_d_t_h.html

	pub fn set_bandwidth(&self, module: bladerf_module, bandwidth: u32) -> Result<u32, isize> {
		let mut actual: u32 = 0;

		unsafe {
			let res = bladerf_set_bandwidth(self.device.assume_init(), module, bandwidth, &mut actual);

			handle_res!(res, actual);
		}
	}

	pub fn get_bandwidth(&self, module: bladerf_module) -> Result<u32, isize> {
		unsafe {
			let mut bandwidth: u32 = 0;

			let res = bladerf_get_bandwidth(self.device.assume_init(), module, &mut bandwidth);

			handle_res!(res, bandwidth);
		}
	}

	pub fn set_lpf_mode(&self, module: bladerf_module, lpf_mode: bladerf_lpf_mode) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_lpf_mode(self.device.assume_init(), module, lpf_mode);

			handle_res!(res);
		}
	}

	pub fn get_lpf_mode(&self, module: bladerf_module) -> Result<bladerf_lpf_mode, isize> {
		unsafe {
			let mut lpf_mode = bladerf_lpf_mode::BLADERF_LPF_NORMAL;

			let res = bladerf_get_lpf_mode(
				self.device.assume_init(),
				module,
				&mut lpf_mode
			);

			handle_res!(res, lpf_mode);
		}
	}

//bladerf_set_bandwidth (struct bladerf *dev, bladerf_module module, unsigned int bandwidth, unsigned int *actual)
//bladerf_get_bandwidth (struct bladerf *dev, bladerf_module module, unsigned int *bandwidth)
//bladerf_set_lpf_mode (struct bladerf *dev, bladerf_module module, bladerf_lpf_mode mode)
//bladerf_get_lpf_mode (struct bladerf *dev, bladerf_module module, bladerf_lpf_mode *mode)

	// Frequency Tuning
	// http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___t_u_n_i_n_g.html

	pub fn select_band(&self, module: bladerf_module, frequency: u32) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_select_band(self.device.assume_init(), module, frequency);

			handle_res!(res);
		}
	}

	pub fn set_frequency(&self, module: bladerf_module, frequency: u64) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_frequency(self.device.assume_init(), module, frequency);

			handle_res!(res);
		}
	}
	 
	pub fn schedule_retune(&self, module: bladerf_module, time: u64, frequency: u32, quick_tune: Option<Struct_bladerf_quick_tune>) -> Result<isize, isize> {
		unsafe {

			let mut quick_tune_int: Struct_bladerf_quick_tune;
			let p: *mut Struct_bladerf_quick_tune;

			// Check whether quick tune exists and map pointer as appropriate
			match quick_tune {
				Some(qt) => {
					quick_tune_int = qt;
					p = &mut quick_tune_int;
				},
				None => {
					p = ptr::null_mut();
				}
			}

			// Call underlying function
			let res = bladerf_schedule_retune(self.device.assume_init(), module, time, frequency, p);

			// Process response
			handle_res!(res)
		}
	}

	pub fn cancel_scheduled_retune(&self, module: bladerf_module) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_cancel_scheduled_retunes(self.device.assume_init(), module) as isize;

			handle_res!(res);
		}
	}

	pub fn get_frequency(&self, module: bladerf_module) -> Result<u64, isize> {
		unsafe {
			let mut freq: u64 = 0;

			let res = bladerf_get_frequency(self.device.assume_init(), module, &mut freq);

			handle_res!(res, freq);
		}
	}

	pub fn get_quick_tune(&self, module: bladerf_module) -> Result<Struct_bladerf_quick_tune, isize> {
		unsafe {
			let mut quick_tune = Struct_bladerf_quick_tune {
				freqsel: 0,
				vcocap: 0,
				nint: 0,
				nfrac: 0,
				flags: 0,
			};

			let res = bladerf_get_quick_tune(self.device.assume_init(), module, &mut quick_tune); 

			handle_res!(res, quick_tune);
		}
	}

	pub fn set_tuning_mode(&self, mode: bladerf_tuning_mode) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_tuning_mode(self.device.assume_init(), mode) as isize;

			handle_res!(res);
		}
	}


	// Internal Loopback
	// http://www.nuand.com/libbladeRF-doc/v1.7.2/group___f_n___l_o_o_p_b_a_c_k.html

	pub fn set_loopback(&self, loopback: bladerf_loopback) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_loopback(self.device.assume_init(), loopback); 

			handle_res!(res);
		}
	}

	pub fn get_loopback(&self) -> Result<bladerf_loopback, isize> {
		unsafe {
			let mut loopback = bladerf_loopback::BLADERF_LB_NONE;
			let res = bladerf_get_loopback(self.device.assume_init(), &mut loopback);
			handle_res!(res, loopback);			
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

	pub fn sync_config(&self, module: bladerf_module, format: bladerf_format,
					   num_buffers: u32, buffer_size: u32, num_transfers: Option<u32>, stream_timeout: u32)
					   -> Result<isize, isize> {

		let num_transfers = match num_transfers { Some(t) => t, None => 4};

		unsafe {
			let res = bladerf_sync_config(self.device.assume_init(), module, format, num_buffers, buffer_size, num_transfers, stream_timeout);
		
			handle_res!(res);
		}
	}

	pub fn sync_tx(&self, data: Vec<iq>, meta: Option<Struct_bladerf_metadata>, stream_timeout: u32)
		       -> Result<isize, isize> {

		// Handle optional meta argument
		let meta_ptr: *mut Struct_bladerf_metadata = match meta { 
			Some(m) => {
				let mut meta_int = m;
				&mut meta_int
			}, None => {
				ptr::null_mut()
			}
		};

		let data_ptr: *mut libc::c_void = data.as_ptr() as *mut libc::c_void;

		unsafe {
			let res = bladerf_sync_tx(self.device.assume_init(), data_ptr, data.len() as u32, meta_ptr, stream_timeout);
		
			handle_res!(res);
		}
	}

	pub fn sync_rx(&self, data: &mut Vec<iq>, meta: Option<Struct_bladerf_metadata>, stream_timeout: u32)
		       -> Result<isize, isize> {

		// Handle optional meta argument
		let meta_ptr: *mut Struct_bladerf_metadata = match meta { 
			Some(m) => {
				let mut meta_int = m;
				&mut meta_int
			}, None => {
				ptr::null_mut()
			}
		};

		let data_ptr: *mut libc::c_void = data.as_ptr() as *mut libc::c_void;

		unsafe {
			let res = bladerf_sync_rx(self.device.assume_init(), data_ptr, data.len() as u32, meta_ptr, stream_timeout);
		
			handle_res!(res);
		}
	}

	// Device loading and programming

	pub fn load_fpga(&self, file: String) -> Result<isize, isize>  {
		let c_string = ffi::CString::new(file.into_bytes()).unwrap();

		unsafe {
			let res = bladerf_load_fpga(self.device.assume_init(), c_string.as_ptr());

			handle_res!(res)
		}
	}


	// Higher level control
	pub fn configure_module(&self, module: bladerf_module, config: BladeRFModuleConfig) {
		BladeRFDevice::set_frequency(self, module, config.frequency).unwrap();
		BladeRFDevice::set_sample_rate(self, module, config.sample_rate).unwrap();
		BladeRFDevice::set_bandwidth(self, module, config.bandwidth).unwrap();
		BladeRFDevice::set_lna_gain(self, config.lna_gain).unwrap();
		match module {
			bladerf_module::BLADERF_MODULE_RX => {
				BladeRFDevice::set_rxvga1(self, config.vga1).unwrap();
				BladeRFDevice::set_rxvga2(self, config.vga2).unwrap();
			},
			bladerf_module::BLADERF_MODULE_TX => {
				BladeRFDevice::set_txvga1(self, config.vga1).unwrap();
				BladeRFDevice::set_txvga2(self, config.vga2).unwrap();
			}
		};
		
	}


}


#[cfg(test)]
mod tests {
	use bladerf::*;

	#[test]
	fn test_list_devices() {
		match super::get_device_list() {
			Ok(devices) => {
				println!("Discovered {:?} devices", devices.len());
			},
			Err(code) => {
				println!("Error {:?} listing devices", code);
				assert!(false);
			}
		}
	}

	#[test]
	fn test_open() {
		let _device = super::open(None).unwrap();
	}

	#[test]
	fn test_open_devinfo() {
		let devices = super::get_device_list().unwrap();
		assert!(devices.len() != 0);
		let _device = super::open_with_devinfo(&devices[0]).unwrap();
	}

	#[test]
	fn test_get_fw_version() {
		let device = super::open(None).unwrap();

		let version = device.fw_version().unwrap();
		println!("FW Version {:?}", version);
	}

	#[test]
	fn test_get_fpga_version() {
		let device = super::open(None).unwrap();

		let version = device.fpga_version().unwrap();
		println!("FPGA Version {:?}", version);
	}

	#[test]
	fn test_get_serial() {
		let device = super::open(None).unwrap();

		let serial = device.get_serial().unwrap();
		println!("Serial: {:?}", serial);
		assert!(serial.len() == 33);
	}

	#[test]
	fn test_fpga_loaded() {
		let device = super::open(None).unwrap();
		
		let loaded = device.is_fpga_configured().unwrap();
		assert_eq!(true, loaded);
	}

	#[test]
	fn test_loopback_modes() {
		let device = super::open(None).unwrap();

		// Check initial is none
		let loopback = device.get_loopback().unwrap();
		assert!(loopback == bladerf_loopback::BLADERF_LB_NONE);

		// Set and check loopback modes
		device.set_loopback(bladerf_loopback::BLADERF_LB_FIRMWARE).unwrap();
		let loopback = device.get_loopback().unwrap();
		assert!(loopback == bladerf_loopback::BLADERF_LB_FIRMWARE);

		// Reset
		device.set_loopback(bladerf_loopback::BLADERF_LB_NONE).unwrap();

		let loopback = device.get_loopback().unwrap();
		assert!(loopback == bladerf_loopback::BLADERF_LB_NONE);
	}

	#[test]
	fn test_set_freq() {
		let device = super::open(None).unwrap();

		let freq: u64 = 915000000;

		// Set and check frequency
		device.set_frequency(bladerf_module::BLADERF_MODULE_RX, freq).unwrap();
		let actual_freq = device.get_frequency(bladerf_module::BLADERF_MODULE_RX).unwrap();
		let diff = freq as i64 - actual_freq as i64;
		assert!(i64::abs(diff) < 10);
	}

	#[test]
	fn test_set_sampling() {
		let device = super::open(None).unwrap();

		let sampling: bladerf_sampling = bladerf_sampling::BLADERF_SAMPLING_INTERNAL;

		// Set and check frequency
		match device.set_sampling(sampling) {
			Ok(_) => (),
			Err(err) => if err != -8 {
				panic!("unexpected error of value when calling set_sampling {:?}", err);
			} else {
				return;				
			}
		};

		let actual_sampling = device.get_sampling().unwrap();

		assert!(actual_sampling == sampling);
	}
}
