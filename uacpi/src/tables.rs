use core::{ffi::c_char, mem::transmute};

#[cfg(feature = "allocator_api")]
use alloc::alloc::Allocator;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::status::UacpiError;

pub mod hpet;

const ACPI_TABLE_HEADER_SIZE: isize = 36;
const ACPI_TABLE_REVISION_OFFSET: isize = 8;
const ACPI_TABLE_OEMID_OFFSET: isize = 10;
const ACPI_TABLE_OEM_TABLE_ID_OFFSET: isize = 16;
const ACPI_TABLE_OEM_REVISION_OFFSET: isize = 24;
const ACPI_TABLE_CREATORID_OFFSET: isize = 28;
const ACPI_TABLE_CREATOR_REVISION_OFFSET: isize = 32;

struct AcpiTableStruct {
    table_ptr: *mut u8,
    index: u64
}

pub trait AcpiTable: AcpiTableInternal + Sized {

    const TABLE_SIGNATURE: [c_char;4];

    fn try_new(index: u64) -> Result<Option<Self>, UacpiError> {
        todo!();
    }
    
    fn table_count() -> u64 {
        todo!();
    }
    
    #[cfg(any(feature = "alloc", feature = "std"))]
    fn try_new_all() -> Result<Box<[Self]>, UacpiError> {
        todo!();
    }
    
    #[cfg(feature = "allocator_api")]
    fn try_new_all_in<A: Allocator>() -> Result<Box<[Self], A>, UacpiError> {
        todo!();
    }

    fn get_revision(&self) -> u8 {
        unsafe { self.get_table_ptr().byte_offset(ACPI_TABLE_REVISION_OFFSET).read() }
    }

    //TODO header getter


}


trait AcpiTableInternal {

    ///Returns the ptr to the table
    fn get_table_ptr(&self) -> *mut u8;
    fn get_index(&self) -> u64;

    ///returns a pointer to the first byte after the header
    fn get_ptr_to_table_payload(&self) -> *mut u8{
        unsafe { self.get_table_ptr().byte_offset(ACPI_TABLE_HEADER_SIZE) }
    }

}

impl AcpiTableInternal for AcpiTableStruct {
    fn get_table_ptr(&self) -> *mut u8 {
        self.table_ptr
    }

    fn get_index(&self) -> u64 {
        self.index
    }
}

unsafe impl Send for AcpiTableStruct {}
unsafe impl Sync for AcpiTableStruct {}

impl Clone for AcpiTableStruct {
    fn clone(&self) -> Self {
        unsafe {
            uacpi_sys::uacpi_table_ref(self as *mut uacpi_sys::uacpi_table);
        }
        Self { table_ptr: self.table_ptr.clone(), index: self.index.clone() }
    }
}

impl Drop for AcpiTableStruct {
    fn drop(&mut self) {
        unsafe {
            uacpi_sys::uacpi_table_unref(self as *mut uacpi_sys::uacpi_table);
        }
    }
}