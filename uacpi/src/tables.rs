use core::{ffi::c_char, mem::MaybeUninit};
use crate::{status::{Status, UacpiError}, tables::hpet::HPET};
use either::Either;

#[cfg(feature = "alloc")]
use alloc::{alloc::AllocError, boxed::Box};

#[cfg(feature = "allocator_api")]
use alloc::alloc::Allocator;



//Add new Tables here
pub mod template;
pub mod hpet;


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AcpiHeader {
    signature: [c_char;4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [c_char;6],
    oem_table_id: [c_char;8],
    oem_revision: u32,
    asl_compiler_id: [c_char;4],
    asl_compiler_revision: u32

}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AcpiTableStruct {
    table_ptr: *mut u8,
    index: u64
}


impl Clone for AcpiTableStruct {
    fn clone(&self) -> Self {
        Status::evaluate_uacpi_status(unsafe {
            uacpi_table_ref(self as *const AcpiTableStruct as *mut u8 as *mut uacpi_sys::uacpi_table)
        }).unwrap();
        Self { table_ptr: self.table_ptr, index: self.index }
    }
}

impl Drop for AcpiTableStruct {
    fn drop(&mut self) {
        Status::evaluate_uacpi_status(unsafe {
            uacpi_table_unref(self as *mut AcpiTableStruct as *mut u8 as *mut uacpi_sys::uacpi_table)
        }).unwrap();
    }
}


trait AcpiTableInternal {

    fn internal_acpi_table_to_self(table: AcpiTableStruct ) -> Self where Self: Sized;

    fn internal_get_acpi_table(&self) -> &AcpiTableStruct;

    /// Returns a ptr to the first byte after the generic table header
    fn internal_start_of_data(&self) -> *const u8;

}

//Used for special tables which have only one instance gurranteed by the spec (FACT,..)
pub trait AcpiTableSingle: AcpiTable {

    fn get() -> Result<Self, UacpiError> where Self: Sized;

}

pub trait AcpiTableMulti: AcpiTable {

    fn try_new() -> Result<Self, UacpiError> where Self: Sized {
        
        let mut table: MaybeUninit<AcpiTableStruct> = MaybeUninit::uninit();

        Status::evaluate_uacpi_status(unsafe {
            uacpi_table_find_by_signature(&Self::TABLE_SIGNATURE as *const c_char, table.as_mut_ptr() as *mut u8 as *mut uacpi_sys::uacpi_table)
        })?;

        Ok(Self::internal_acpi_table_to_self(unsafe { table.assume_init() }))
    }

    fn try_new_next(&self) -> Result<Self, UacpiError> where Self: Sized {
        
        let mut next_table = self.internal_get_acpi_table().clone();

        Status::evaluate_uacpi_status(unsafe {
            uacpi_table_find_next_with_same_signature(&mut next_table as *mut AcpiTableStruct as *mut u8 as *mut uacpi_sys::uacpi_table)
        })?;

        Ok(Self::internal_acpi_table_to_self(next_table))
    }

    fn get_number_tables() -> usize {

        let mut table: MaybeUninit<AcpiTableStruct> = MaybeUninit::uninit();

        match Status::evaluate_uacpi_status(unsafe {
            uacpi_table_find_by_signature(&Self::TABLE_SIGNATURE as *const c_char, table.as_mut_ptr() as *mut u8 as *mut uacpi_sys::uacpi_table)
        }) {
            Ok(_) => {},
            Err(_) => return 0,
        }

        let mut counter: usize = 1;

        loop {
            
            match Status::evaluate_uacpi_status(unsafe {
                uacpi_table_find_next_with_same_signature(table.as_mut_ptr() as *mut u8 as *mut uacpi_sys::uacpi_table)
            }) {
                Ok(_) => counter += 1,
                Err(_) => return counter,
            }
        }

    }

    /// # Result
    /// () -> Slice to small
    fn try_new_slice(slice: &mut [MaybeUninit<Self>]) -> Result<&[Self], Either<(),UacpiError>> where Self: Sized {

        //Check if slice is large enough
        if slice.len() < Self::get_number_tables() {
            return Err(Either::Left(()));
        }

        let sub_slice = &mut slice[..Self::get_number_tables()];
        //Fill slice

        sub_slice[0].write(Self::try_new().map_err(|error| { Either::Right(error)})?);

        for n in 1..Self::get_number_tables() {

            sub_slice[n].write(
                Self::try_new_next(unsafe { sub_slice[n-1].assume_init_ref() }).map_err(|error| { Either::Right(error)})?
            );

        }


        Ok(unsafe { sub_slice.assume_init_mut() })

    }

    #[cfg(feature = "alloc")]
    fn try_new_all() -> Result<Box<[Self]>, Either<AllocError,UacpiError>> where Self: Sized {
        
        let mut storage: Box<[MaybeUninit<Self>]> = Box::try_new_uninit_slice(Self::get_number_tables()).map_err(|error| { Either::Left(error)})?;

        storage[0].write(Self::try_new().map_err(|error| { Either::Right(error)})?);

        for n in 1..Self::get_number_tables() {

            storage[n].write(
                Self::try_new_next(unsafe { storage[n-1].assume_init_ref() }).map_err(|error| { Either::Right(error)})?
            );

        }

        Ok(unsafe { storage.assume_init() })

    }

    #[cfg(feature = "allocator_api")]
    fn try_new_all_in<A: Allocator>(allocator: A) -> Result<Box<[Self], A>, Either<AllocError,UacpiError>> where Self: Sized {
        
        let mut storage: Box<[MaybeUninit<Self>], A> = Box::try_new_uninit_slice_in(Self::get_number_tables(), allocator).map_err(|error| { Either::Left(error)})?;

        storage[0].write(Self::try_new().map_err(|error| { Either::Right(error)})?);

        for n in 1..Self::get_number_tables() {

            storage[n].write(
                Self::try_new_next(unsafe { storage[n-1].assume_init_ref() }).map_err(|error| { Either::Right(error)})?
            );

        }

        Ok(unsafe { storage.assume_init() })

    }

}

pub trait AcpiTable: AcpiTableInternal {

    const TABLE_SIGNATURE: [c_char;4];

    fn get_signature(&self) -> [core::ffi::c_char;4] {
        (unsafe { &*(Self::internal_get_acpi_table(self).table_ptr as *mut AcpiHeader) }).signature
    }

    fn get_length(&self) -> u32 {
        (unsafe { &*(Self::internal_get_acpi_table(self).table_ptr as *mut AcpiHeader) }).length
    }

    fn get_revision(&self) -> u8 {
        (unsafe { &*(Self::internal_get_acpi_table(self).table_ptr as *mut AcpiHeader) }).revision
    }

    fn get_oem_id(&self) -> [core::ffi::c_char;6] {
        (unsafe { &*(Self::internal_get_acpi_table(self).table_ptr as *mut AcpiHeader) }).oem_id
    }

    fn get_oem_table_id(&self) -> [core::ffi::c_char;8] {
        (unsafe { &*(Self::internal_get_acpi_table(self).table_ptr as *mut AcpiHeader) }).oem_table_id
    }

    fn get_oem_revision(&self) -> u32 {
        (unsafe { &*(Self::internal_get_acpi_table(self).table_ptr as *mut AcpiHeader) }).oem_revision
    }

    fn get_asl_compiler_id(&self) -> [core::ffi::c_char;4] {
        (unsafe { &*(Self::internal_get_acpi_table(self).table_ptr as *mut AcpiHeader) }).asl_compiler_id
    }

    fn get_asl_compiler_revision(&self) -> u32 {
        (unsafe { &*(Self::internal_get_acpi_table(self).table_ptr as *mut AcpiHeader) }).asl_compiler_revision
    }


}

macro_rules! AcpiTableSignature {
    ($lit:literal) => {{
        const BYTES: &[u8] = $lit;
        // compile-time length check
        let _ = [(); 4 - BYTES.len()];
        let _ = [(); BYTES.len() - 4];

        [
            BYTES[0] as c_char,
            BYTES[1] as c_char,
            BYTES[2] as c_char,
            BYTES[3] as c_char,
        ]
    }};
}

pub(super) use AcpiTableSignature;
use uacpi_sys::{uacpi_table_find_by_signature, uacpi_table_find_next_with_same_signature, uacpi_table_ref, uacpi_table_unref};




pub enum AcpiTables {
    HPET(HPET)
    //Add new Tables here
}
