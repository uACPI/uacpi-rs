use core::ffi::{c_void, CStr};
use crate::{NamespaceNode, Status};

extern crate alloc;

use alloc::vec::Vec;
use core::slice;

#[repr(C)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum NsIterDecision {
	Continue,
	NextPeer,
	Break
}

#[repr(C)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum InterruptModel {
	Pic,
	IoApic,
	IoSapic
}

#[repr(transparent)]
pub struct IdString(pub(crate) *mut uacpi_sys::uacpi_id_string);

impl Drop for IdString {
	fn drop(&mut self) {
		unsafe {
			uacpi_sys::uacpi_free_id_string(self.0);
		}
	}
}

impl IdString {
	pub fn as_str(&self) -> &str {
		unsafe {
			let slice = slice::from_raw_parts(
				(*self.0).value as *const u8,
				(*self.0).size as usize
			);
			CStr::from_bytes_with_nul(slice).unwrap()
		}.to_str().unwrap()
	}
}

/// Checks whether the device at `node` matches any of the PNP ids provided in `list`.
/// This is done by first attempting to match the value returned from _HID
/// and then the value(s) from _CID.
/// Note that the presence of the device (_STA) is not verified here.
pub fn device_matches_pnp_id(node: &NamespaceNode, list: &[&CStr]) -> bool {
	let mut vec: Vec<_> = list.iter().map(|str| str.as_ptr()).collect();
	vec.push(core::ptr::null());
	unsafe {
		uacpi_sys::uacpi_device_matches_pnp_id(node.0, vec.as_ptr())
	}
}

unsafe extern "C" fn uacpi_iter_cb<F: FnMut(&NamespaceNode) -> NsIterDecision>(
	user: *mut c_void,
	node: *mut uacpi_sys::uacpi_namespace_node
) -> uacpi_sys::uacpi_ns_iteration_decision {
	let f = user as *mut F;
	(*f)(*(node as *mut _)) as _
}

/// Finds all the devices in the namespace starting at `parent` matching the
/// specified `hids`. Only devices reported as present via _STA are checked.
/// Any matching devices are then passed to the `cb`.
pub fn find_devices_at<F: FnMut(&NamespaceNode) -> NsIterDecision>(
	parent: &NamespaceNode,
	hids: &[&CStr],
	cb: F
) -> Result<(), Status> {
	let mut vec: Vec<_> = hids.iter().map(|str| str.as_ptr()).collect();
	vec.push(core::ptr::null());
	let status: Status = unsafe {
		uacpi_sys::uacpi_find_devices_at(
			parent.0,
			vec.as_ptr(),
			Some(uacpi_iter_cb::<F>),
			&cb as *const _ as _
		).into()
	};

	match status {
		Status::Ok => Ok(()),
		_ => Err(status)
	}
}

/// Same as find_devices_at, except this starts at the root and only
/// matches one hid.
pub fn find_devices<F: FnMut(&NamespaceNode) -> NsIterDecision>(
	hid: &CStr,
	cb: F
) -> Result<(), Status> {
	let status: Status = unsafe {
		uacpi_sys::uacpi_find_devices(
			hid.as_ptr(),
			Some(uacpi_iter_cb::<F>),
			&cb as *const _ as _
		).into()
	};

	match status {
		Status::Ok => Ok(()),
		_ => Err(status)
	}
}

/// Sets the currently active interrupt model.
pub fn set_interrupt_model(model: InterruptModel) -> Result<(), Status> {
	let status: Status = unsafe {
		uacpi_sys::uacpi_set_interrupt_model(
			model as _
		).into()
	};

	match status {
		Status::Ok => Ok(()),
		_ => Err(status)
	}
}

/// Evaluate a device's _HID method and get its value.
pub fn eval_hid(node: &NamespaceNode) -> Result<IdString, Status> {
	let mut ret = core::ptr::null_mut();
	let status: Status = unsafe {
		uacpi_sys::uacpi_eval_hid(
			node.0,
			&mut ret
		).into()
	};

	match status {
		Status::Ok => Ok(IdString(ret)),
		_ => Err(status)
	}
}
