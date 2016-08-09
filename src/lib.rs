extern crate libc;

use std::*;

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
	pub frequency: u32,
	pub sample_rate: u32,
	pub bandwidth: u32,
	pub lna_gain: u32,
	pub vga1: u32,
	pub vga2: u32
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
   device: *mut Struct_bladerf
}


/***		Static Functions			***/

pub fn get_device_list() -> Result<Vec<Struct_bladerf_devinfo>, isize> {

	unsafe{ 
		let devices: *mut Struct_bladerf_devinfo = mem::uninitialized();

		let n = bladerf_get_device_list(&devices) as isize;

		// Catch bladerf function errors
		if n > 0 {

			// Cast array to slice and create a safe array to return
			let device_slice = std::slice::from_raw_parts(devices, n as usize);
			let mut safe_device_list: Vec<Struct_bladerf_devinfo> = Vec::new();

			for i in 0..n {
				let local_device = device_slice[i as usize];
				//Safe if this is a copy, unsafe if it is not?
				safe_device_list.push(local_device);
			}
			bladerf_free_device_list(devices);
			
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

		let bladerf_device = BladeRFDevice { device: mem::uninitialized() };

		let res = bladerf_open(&(bladerf_device.device), id_ptr);

		handle_res!(res, bladerf_device);
	}
}

pub fn open_with_devinfo(devinfo: &Struct_bladerf_devinfo) -> Result<BladeRFDevice, isize> {

	let devinfo_ptr: *const Struct_bladerf_devinfo = devinfo as *const Struct_bladerf_devinfo;

	unsafe {
		let bladerf_device = BladeRFDevice { device: mem::uninitialized() };

		let res = bladerf_open_with_devinfo(&(bladerf_device.device), devinfo_ptr);

		handle_res!(res, bladerf_device);
	}
}


/***		BladeRFDevice Methods			***/

impl BladeRFDevice {

	pub fn fw_version(&self) -> Result<Struct_bladerf_version, isize> {
		unsafe {
			let mut version: Struct_bladerf_version = mem::uninitialized();

			let res = bladerf_fw_version(self.device, &mut version as *mut Struct_bladerf_version);

			handle_res!(res, version);
		}
	}

	pub fn fpga_version(&self) -> Result<Struct_bladerf_version, isize> {
		unsafe {
			let mut version: Struct_bladerf_version = mem::uninitialized();

			let res = bladerf_fpga_version(self.device, &mut version as *mut Struct_bladerf_version);

			handle_res!(res, version);
		}
	}

	pub fn get_serial(&self) -> Result<String, isize> {
		unsafe {
			// Create raw data array for serial return
			let serial_data : Vec<::libc::c_char> = vec![0; 33];

			// Call underlying c method
			let res = bladerf_get_serial(self.device, serial_data.as_ptr());

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
			let res = bladerf_get_fpga_size(self.device, &mut fpga_size as *mut bladerf_fpga_size);

			handle_res!(res, fpga_size);
		}
	}

	pub fn is_fpga_configured(&self) -> Result<bool, isize> {
		unsafe {
			let res = bladerf_is_fpga_configured(self.device);

			if res > 0 {
				Ok(true)
			} else if res == 0 {
				Ok(false)
			} else {
				Err(res as isize)
			}
		}
	}

	pub fn load_fpga(&self, file: String) -> Result<isize, isize>  {
		let c_string = ffi::CString::new(file.into_bytes()).unwrap();

		unsafe {
			let res = bladerf_load_fpga(self.device, c_string.as_ptr());

			handle_res!(res)
		}
	}


	pub fn close(&self) {
		unsafe {
			bladerf_close(self.device)
		}
	}


	pub fn enable_module(&self, module: bladerf_module, enable: bool) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_enable_module(self.device, module, enable as u8);

			handle_res!(res);
		}
	}

	pub fn set_loopback(&self, loopback: bladerf_loopback) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_loopback(self.device, loopback); 

			handle_res!(res);
		}
	}

	pub fn get_loopback(&self) -> Result<bladerf_loopback, isize> {
		unsafe {
			let mut loopback: bladerf_loopback = mem::uninitialized();

			let res = bladerf_get_loopback(self.device, &mut loopback as *mut bladerf_loopback); 

			handle_res!(res, loopback);
		}
	}

	pub fn select_band(&self, module: bladerf_module, frequency: u32) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_select_band(self.device, module, frequency);

			handle_res!(res);
		}
	}

	pub fn set_frequency(&self, module: bladerf_module, frequency: u32) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_frequency(self.device, module, frequency);

			handle_res!(res);
		}
	}

	pub fn get_frequency(&self, module: bladerf_module) -> Result<u32, isize> {
		unsafe {
			let mut freq: u32 = 0;

			let res = bladerf_get_frequency(self.device, module, &mut freq as *mut u32); 

			handle_res!(res, freq);
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
			let res = bladerf_schedule_retune(self.device, module, time, frequency, p);

			// Process response
			handle_res!(res)
		}
	}

	pub fn cancel_scheduled_retune(&self, module: bladerf_module) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_cancel_scheduled_retunes(self.device, module) as isize;

			handle_res!(res);
		}
	}

	pub fn get_quick_tune(&self, module: bladerf_module) -> Result<Struct_bladerf_quick_tune, isize> {
		unsafe {
			let mut quick_tune: Struct_bladerf_quick_tune = mem::uninitialized();

			let res = bladerf_get_quick_tune(self.device, module, &mut quick_tune as *mut Struct_bladerf_quick_tune); 

			handle_res!(res, quick_tune);
		}
	}

	pub fn set_tuning_mode(&self, mode: bladerf_tuning_mode) -> Result<isize, isize> {
		unsafe {
			let res = bladerf_set_tuning_mode(self.device, mode) as isize;

			handle_res!(res);
		}
	}

	pub fn sync_config(&self, module: bladerf_module, format: bladerf_format,
					   num_buffers: u32, buffer_size: u32, num_transfers: Option<u32>, stream_timeout: u32)
					   -> Result<isize, isize> {

		let num_transfers = match num_transfers { Some(t) => t, None => 4};

		unsafe {
			let res = bladerf_sync_config(self.device, module, format, num_buffers, buffer_size, num_transfers, stream_timeout);
		
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
			let res = bladerf_sync_tx(self.device, data_ptr, data.len() as u32, meta_ptr, stream_timeout);
		
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
			let res = bladerf_sync_rx(self.device, data_ptr, data.len() as u32, meta_ptr, stream_timeout);
		
			handle_res!(res);
		}
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
		let device = super::open(None).unwrap();
		device.close();
	}

	#[test]
	fn test_open_devinfo() {
		let devices = super::get_device_list().unwrap();
		assert!(devices.len() != 0);
		let device = super::open_with_devinfo(&devices[0]).unwrap();
		device.close();
	}

	#[test]
	fn test_get_fw_version() {
		let device = super::open(None).unwrap();

		let version = device.fw_version().unwrap();
		println!("FW Version {:?}", version);

		device.close();
	}

	#[test]
	fn test_get_fpga_version() {
		let device = super::open(None).unwrap();

		let version = device.fpga_version().unwrap();
		println!("FPGA Version {:?}", version);

		device.close();
	}

	#[test]
	fn test_get_serial() {
		let device = super::open(None).unwrap();

		let serial = device.get_serial().unwrap();
		println!("Serial: {:?}", serial);
		assert!(serial.len() == 33);

		device.close();
	}

	#[test]
	fn test_fpga_loaded() {
		let device = super::open(None).unwrap();
		
		let loaded = device.is_fpga_configured().unwrap();
		assert_eq!(true, loaded);

		device.close();
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

		device.close();
	}
}
