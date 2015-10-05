extern crate libc;

use std::*;
use libc::*;

#[allow(dead_code, non_camel_case_types)]
mod bladerf;
use bladerf::*;

impl fmt::Debug for Struct_bladerf_devinfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "serial: UNIMPLEMENTED, bus: {}, address: {})", self.usb_bus, self.usb_addr)
    }
}

//#[link(name = "bladerf")]
//extern {
//	fn bladerf_get_device_list(devices: &*mut [Struct_bladerf_devinfo]) -> libc::c_int;
//	fn bladerf_free_device_list(devices: *mut [Struct_bladerf_devinfo]);
//    fn bladerf_set_usb_reset_on_open (enabled: bool);
//}

pub fn get_device_list() -> Result<Vec<Struct_bladerf_devinfo>, isize> {

	unsafe{ 
		let devices: *mut [Struct_bladerf_devinfo] = mem::uninitialized();

		let n = bladerf_get_device_list(&devices) as isize;

		let mut safe_device_list: Vec<Struct_bladerf_devinfo> = Vec::new();

		for i in 0..n {
			let local_device = (*devices)[i as usize];
			//Safe if this is a copy, unsafe if it is not?
			safe_device_list.push(local_device);
		}

		bladerf_free_device_list(devices);

		if n >= 0 {
			Ok(safe_device_list)
		} else {
			Err(n)
		}
	}
}

pub fn set_usb_reset_on_open(enabled: bool) {
    unsafe{ 
    	bladerf_set_usb_reset_on_open(enabled as uint8_t); 
    } 
}

pub fn open_with_devinfo(devinfo: &Struct_bladerf_devinfo) -> Result<isize, isize> {
	unsafe {
		let device: *mut *mut Struct_bladerf = mem::uninitialized();
		let unsafe_devinfo: *const Struct_bladerf_devinfo = devinfo as *const Struct_bladerf_devinfo;

		let res = bladerf_open_with_devinfo(device, unsafe_devinfo) as isize;

		if res >= 0 {
			Ok(res)
		} else {
			Err(res)
		}
	}
}

pub fn fw_version(dev: *mut Struct_bladerf) -> Result<*mut Struct_bladerf_version, isize> {
	unsafe {
		let version: *mut Struct_bladerf_version = mem::uninitialized();;
		let res = bladerf_fw_version(dev, version) as isize;

		if res >= 0 {
			Ok(version)
		} else {
			Err(res)
		}
	}
}


pub fn fpga_version(dev: *mut Struct_bladerf) -> Result<*mut Struct_bladerf_version, isize> {
	unsafe {
		let version: *mut Struct_bladerf_version = mem::uninitialized();;
		let res = bladerf_fpga_version(dev, version) as isize;

		if res >= 0 {
			Ok(version)
		} else {
			Err(res)
		}
	}
}

pub fn close_device(device: *mut Struct_bladerf) {
	unsafe {
		bladerf_close(device)
	}
}

#[test]
fn discovery() {
	match get_device_list() {
		Ok(devices) => {
			println!("Discovered {} devices", devices);
		},
		Err(code) => {
			println!("Error {} listing devices", code);
			assert!(false);
		}
	}
}
