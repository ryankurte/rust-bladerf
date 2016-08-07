extern crate libc;

use std::*;
use libc::*;

#[allow(dead_code, non_camel_case_types)]
mod bladerf;
use bladerf::*;

pub struct BladerfDevice {
    pub device: *mut *mut Struct_bladerf,
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

		if res >= 0 {
			Ok(device_ptr)
		} else {
			Err(res)
		}
	}
}

pub fn fw_version(dev: *mut Struct_bladerf) -> Result<Struct_bladerf_version, isize> {
	unsafe {
		let mut version: Struct_bladerf_version = mem::uninitialized();

		let res = bladerf_fw_version(dev, &mut version as *mut Struct_bladerf_version) as isize;

		if res >= 0 {
			Ok(version)
		} else {
			Err(res)
		}
	}
}

pub fn fpga_version(dev: *mut Struct_bladerf) -> Result<Struct_bladerf_version, isize> {
	unsafe {
		let mut version: Struct_bladerf_version = mem::uninitialized();

		let res = bladerf_fpga_version(dev, &mut version as *mut Struct_bladerf_version) as isize;

		if res >= 0 {
			Ok(version)
		} else {
			Err(res)
		}
	}
}

pub fn get_serial(dev: *mut Struct_bladerf) -> Result<String, isize> {
	unsafe {
		let serial_data : Vec<::libc::c_char> = vec![0, 33];

		let res = bladerf_get_serial(dev, serial_data.as_ptr()) as isize;
		
		println!("Serial: {:?}", serial_data);

		let serial_u8: Vec<u8>= serial_data.iter().map(|&x| x as u8).collect();
		let serial_cstr = ffi::CString::from_vec_unchecked(serial_u8);
		let serial_str = serial_cstr.into_string().unwrap();

		if res >= 0 {
			Ok(serial_str)
		} else {
			Err(res as isize)
		}
	}
}

pub fn close_device(device: *mut Struct_bladerf) {
	unsafe {
		bladerf_close(device)
	}
}


#[cfg(test)]
mod tests {

	#[test]
	fn list_devices() {
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
	fn get_version() {
		let devices = super::get_device_list().unwrap();
		assert!(devices.len() != 0);
		let device = super::open_with_devinfo(&devices[0]).unwrap();
		let version = super::fw_version(device).unwrap();
		println!("Version {:?}", version);
		super::close_device(device);
	}

	#[test]
	fn get_serial() {
		let devices = super::get_device_list().unwrap();
		assert!(devices.len() != 0);
		let device = super::open_with_devinfo(&devices[0]).unwrap();
		let serial = super::get_serial(device).unwrap();
		println!("Serial: {:?}", serial);
		assert!(serial.len() == 32);
		super::close_device(device);
	}

}
