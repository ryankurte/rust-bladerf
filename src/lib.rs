extern crate libc;

use std::*;
use libc::*;

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

pub struct BladerfDevice {
    pub device: *mut Struct_bladerf,
    pub open: bool
}

#[repr(C)]
#[repr(packed)]
pub struct iq {
	pub i: i16,
	pub q: i16
}

//impl fmt::Display for Struct_bladerf_devinfo {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "serial: UNIMPLEMENTED, bus: {}, address: {})", self.usb_bus, self.usb_addr)
//    }
//}

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
    	bladerf_set_usb_reset_on_open(enabled as uint8_t); 
    } 
}

pub fn open_with_devinfo(devinfo: &Struct_bladerf_devinfo) -> Result<*mut Struct_bladerf, isize> {
	unsafe {
		let device_ptr: *mut Struct_bladerf = mem::uninitialized();
		let unsafe_devinfo: *const Struct_bladerf_devinfo = devinfo as *const Struct_bladerf_devinfo;

		let res = bladerf_open_with_devinfo(&device_ptr, unsafe_devinfo) as isize;

		handle_res!(res, device_ptr);
	}
}

pub fn fw_version(dev: *mut Struct_bladerf) -> Result<Struct_bladerf_version, isize> {
	unsafe {
		let mut version: Struct_bladerf_version = mem::uninitialized();

		let res = bladerf_fw_version(dev, &mut version as *mut Struct_bladerf_version) as isize;

		handle_res!(res, version);
	}
}

pub fn fpga_version(dev: *mut Struct_bladerf) -> Result<Struct_bladerf_version, isize> {
	unsafe {
		let mut version: Struct_bladerf_version = mem::uninitialized();

		let res = bladerf_fpga_version(dev, &mut version as *mut Struct_bladerf_version) as isize;

		handle_res!(res, version);
	}
}

pub fn get_serial(dev: *mut Struct_bladerf) -> Result<String, isize> {
	unsafe {
		// Create raw data array for serial return
		let serial_data : Vec<::libc::c_char> = vec![0; 33];

		// Call underlying c method
		let res = bladerf_get_serial(dev, serial_data.as_ptr()) as isize;

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

pub fn enable_module(device: *mut Struct_bladerf, module: bladerf_module, enable: bool) -> Result<isize, isize> {
	unsafe {
		let res = bladerf_enable_module(device, module, enable as u8) as isize;

		handle_res!(res);
	}
}

pub fn set_loopback(device: *mut Struct_bladerf, loopback: bladerf_loopback) -> Result<isize, isize> {
	unsafe {
		let res = bladerf_set_loopback(device, loopback) as isize; 

		handle_res!(res);
	}
}

pub fn get_loopback(device: *mut Struct_bladerf) -> Result<bladerf_loopback, isize> {
	unsafe {
		let mut loopback: bladerf_loopback = mem::uninitialized();

		let res = bladerf_get_loopback(device, &mut loopback as *mut bladerf_loopback) as isize; 

		println!("get_loopback res: {:?}", res);

		handle_res!(res, loopback);
	}
}

pub fn select_band(device: *mut Struct_bladerf, module: bladerf_module, frequency: u32) -> Result<isize, isize> {
	unsafe {
		let res = bladerf_select_band(device, module, frequency) as isize;

		handle_res!(res);
	}
}

pub fn set_frequency(device: *mut Struct_bladerf, module: bladerf_module, frequency: u32) -> Result<isize, isize> {
	unsafe {
		let res = bladerf_set_frequency(device, module, frequency) as isize;

		handle_res!(res);
	}
}

pub fn get_frequency(device: *mut Struct_bladerf, module: bladerf_module) -> Result<u32, isize> {
	unsafe {
		let mut freq: u32 = 0;

		let res = bladerf_get_frequency(device, module, &mut freq as *mut u32) as isize; 

		handle_res!(res, freq);
	}
}

 
pub fn schedule_retune(device: *mut Struct_bladerf, module: bladerf_module, time: u64, frequency: u32, quick_tune: Option<Struct_bladerf_quick_tune>) -> Result<isize, isize> {
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
		let res = bladerf_schedule_retune(device, module, time, frequency, p) as isize;

		// Process response
		handle_res!(res)
	}
}

pub fn cancel_scheduled_retune(device: *mut Struct_bladerf, module: bladerf_module) -> Result<isize, isize> {
	unsafe {
		let res = bladerf_cancel_scheduled_retunes(device, module) as isize;

		handle_res!(res);
	}
}

pub fn get_quick_tune(device: *mut Struct_bladerf, module: bladerf_module) -> Result<Struct_bladerf_quick_tune, isize> {
	unsafe {
		let mut quick_tune: Struct_bladerf_quick_tune = mem::uninitialized();

		let res = bladerf_get_quick_tune(device, module, &mut quick_tune as *mut Struct_bladerf_quick_tune) as isize; 

		handle_res!(res, quick_tune);
	}
}

pub fn set_tuning_mode(device: *mut Struct_bladerf, mode: bladerf_tuning_mode) -> Result<isize, isize> {
	unsafe {
		let res = bladerf_set_tuning_mode(device, mode) as isize;

		handle_res!(res);
	}
}

pub fn sync_config(device: *mut Struct_bladerf, module: bladerf_module, format: bladerf_format,
				   num_buffers: u32, buffer_size: u32, num_transfers: Option<u32>, stream_timeout: u32)
				   -> Result<isize, isize> {

	let num_transfers = match num_transfers { Some(t) => t, None => 4};

	unsafe {
		let res = bladerf_sync_config(device, module, format, num_buffers, buffer_size, num_transfers, stream_timeout);
	
		handle_res!(res);
	}
}

pub fn sync_tx(device: *mut Struct_bladerf, data: Vec<iq>, meta: Option<Struct_bladerf_metadata>, stream_timeout: u32)
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
		let res = bladerf_sync_tx(device, data_ptr, data.len() as u32, meta_ptr, stream_timeout);
	
		handle_res!(res);
	}
}

pub fn sync_rx(device: *mut Struct_bladerf, data: &mut Vec<iq>, meta: Option<Struct_bladerf_metadata>, stream_timeout: u32)
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
		let res = bladerf_sync_rx(device, data_ptr, data.len() as u32, meta_ptr, stream_timeout);
	
		handle_res!(res);
	}
}


pub fn close_device(device: *mut Struct_bladerf) {
	unsafe {
		bladerf_close(device)
	}
}


#[cfg(test)]
mod tests {
	use super::*;
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
	fn test_get_version() {
		let devices = super::get_device_list().unwrap();
		assert!(devices.len() != 0);
		let device = super::open_with_devinfo(&devices[0]).unwrap();
		let version = super::fw_version(device).unwrap();
		println!("Version {:?}", version);
		super::close_device(device);
	}

	#[test]
	fn test_get_serial() {
		let devices = super::get_device_list().unwrap();
		assert!(devices.len() != 0);
		let device = super::open_with_devinfo(&devices[0]).unwrap();
		let serial = super::get_serial(device).unwrap();
		println!("Serial: {:?}", serial);
		assert!(serial.len() == 33);
		super::close_device(device);
	}

	#[test]
	#[ignore]
	fn test_loopback_modes() {
		let devices = super::get_device_list().unwrap();
		assert!(devices.len() != 0);
		let device = super::open_with_devinfo(&devices[0]).unwrap();

		let loopback = super::get_loopback(device).unwrap();
		assert!(loopback == bladerf_loopback::BLADERF_LB_NONE);

		//super::set_loopback(device, bladerf_loopback::BLADERF_LB_FIRMWARE).unwrap();

		super::close_device(device);
	}
}
