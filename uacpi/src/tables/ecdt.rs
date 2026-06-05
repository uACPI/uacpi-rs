use core::{ffi::{CStr, c_char}};
use crate::{tables::{AcpiHeader, AcpiTable, AcpiTableInternal, AcpiTableSignature, AcpiTableSingle, AcpiTableStruct}, types::{GenericAddressStructure, GenericAddressStructureInternal}};


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ECDT(AcpiTableStruct);

impl ECDT {

    pub fn get_ec_control_gas(&self) -> GenericAddressStructure {

        unsafe { 
            (
                *(self.internal_start_of_data() as *const ECDTInternal)
            ).ec_control 
        }.into()

    }

    pub fn get_ec_data_gas(&self) -> GenericAddressStructure {
        
        unsafe { 
            (
                *(self.internal_start_of_data() as *const ECDTInternal)
            ).ec_data 
        }.into()

    }

    pub fn get_uid(&self) -> u32 {

        unsafe { 
            (
                *(self.internal_start_of_data() as *const ECDTInternal)
            ).uid 
        }

    }

    pub fn get_gpe_bit(&self) -> u8 {

        unsafe { 
            (
                *(self.internal_start_of_data() as *const ECDTInternal)
            ).gpe_bit 
        }

    }

    ///
    /// ## Error
    /// Returns an error if the String is Out of bounds of the table and as such must be considered invalid
    pub fn get_ec_id(&self) -> Result<&CStr,()> {

        let expected_c_string_len = self.get_length() as usize - size_of::<AcpiHeader>() + size_of::<ECDTInternal>();
        let c_cstring_ptr = unsafe {
            &raw const (
                *(
                    self.internal_start_of_data() as *mut ECDTInternal
                )
            ).ec_id
        } as *const c_char;

        if unsafe {
            c_cstring_ptr.add(expected_c_string_len).read() == 0
        } {
            return Ok(unsafe { CStr::from_ptr(c_cstring_ptr) });
        } else {
            return Err(());
        }

    }


}





#[repr(C, packed)]
struct ECDTInternal {
    ec_control: GenericAddressStructureInternal,
    ec_data: GenericAddressStructureInternal,
    uid: u32,
    gpe_bit: u8,
    ec_id: [c_char; 0],
}


impl AcpiTable for ECDT {
    
    const TABLE_SIGNATURE: [c_char;4] = AcpiTableSignature!(b"ECDT"); //<< Change Table Signature here

}

impl AcpiTableInternal for ECDT {
    
    fn internal_acpi_table_to_self(table: AcpiTableStruct ) -> Self where Self: Sized {
        Self(table)
    }
    
    fn internal_get_acpi_table(&self) -> &AcpiTableStruct {
        &self.0
    }
    
    fn internal_start_of_data(&self) -> *const u8 {
        unsafe { self.0.table_ptr.byte_add(size_of::<AcpiHeader>()) }
    }

}

impl AcpiTableSingle for ECDT {}
