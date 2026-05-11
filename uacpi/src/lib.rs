//#![no_std]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

use core::ffi::c_void;

use crate::status::{Status, UacpiError};

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

pub mod kernel_api;
pub mod status;
pub mod types;
pub mod tables;


/*
Replace all *out with MaybeUninit and MaybeIninit.assume_init() style code!

Split the ACPI Table trait into ACPI Table and ACPI Table new, to allow special tables such as fadt
Have three trait:
    ACPI Table
    ACPI Table new single instance (used for tables which have always only one instance such as fadt)
    ACPI Table new multi instance (used for tables wich can have more than one instance)



    Use sub enums extensively to clean up error handling

*/



/// Returns the uACPI Version
/// 
/// ## Returns
/// (Major, Minor, Patch)
pub fn get_uacpi_version() -> (u32, u32, u32) {
    (
        uacpi_sys::UACPI_MAJOR,
        uacpi_sys::UACPI_MINOR,
        uacpi_sys::UACPI_PATCH
    )
}


pub unsafe fn setup_early_table_access(temp_buffer: &mut[u8]) -> Result<(), UacpiError> {
    
    Status::evaluate_uacpi_status(unsafe {
        uacpi_sys::uacpi_setup_early_table_access(temp_buffer.as_mut_ptr() as *mut c_void, temp_buffer.len())
    })

}

pub fn state_reset() {

    unsafe {
        uacpi_sys::uacpi_state_reset();
    }

}