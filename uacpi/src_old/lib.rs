#![no_std]

extern crate alloc;

pub mod kernel_api;
pub mod types;
pub mod namespace;
pub mod sleep;
pub mod tables;
pub mod utils;

use alloc::vec::Vec;
use core::ffi::CStr;
pub use types::*;
pub use namespace::*;
pub use sleep::*;
pub use tables::*;
pub use utils::*;

pub use uacpi_sys as sys;

pub fn init(rsdp: PhysAddr, log_level: LogLevel, no_acpi_mode: bool) -> Result<(), Status> {
    let mut params = uacpi_sys::uacpi_init_params {
        rsdp: rsdp.0,
        log_level: log_level.0,
        flags: if no_acpi_mode { uacpi_sys::UACPI_FLAG_NO_ACPI_MODE as u64 } else { 0 },
    };

    let status: Status = unsafe { uacpi_sys::uacpi_initialize(&mut params).into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status),
    }
}

pub fn namespace_load() -> Result<(), Status> {
    let status: Status = unsafe { uacpi_sys::uacpi_namespace_load().into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status),
    }
}

pub fn namespace_initialize() -> Result<(), Status> {
    let status: Status = unsafe { uacpi_sys::uacpi_namespace_initialize().into() };

    match status {
        Status::Ok => Ok(()),
        _ => Err(status),
    }
}

pub fn get_current_init_level() -> InitLevel {
    unsafe {
        uacpi_sys::uacpi_get_current_init_level().into()
    }
}

pub fn eval<'a>(parent: &NamespaceNode, path: &CStr, args: impl IntoIterator<Item=&'a Object>,
) -> Result<Object, Status> {
    let mut args_vec: Vec<_> = args.into_iter().map(|obj| obj.0).collect();
    let args = uacpi_sys::uacpi_args {
        objects: args_vec.as_mut_ptr(),
        count: args_vec.len(),
    };
    unsafe {
        let mut ret = core::ptr::null_mut();
        let status: Status = uacpi_sys::uacpi_eval(parent.0, path.as_ptr(), &args, &mut ret).into();

        match status {
            Status::Ok => Ok(Object(ret)),
            _ => Err(status)
        }
    }
}
